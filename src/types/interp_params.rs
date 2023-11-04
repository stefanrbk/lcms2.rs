use log::Level;

use crate::{
    consts::MAX_STAGE_CHANNELS,
    inlines::{fixed_rest_to_int, fixed_to_int, quick_floor, round_fixed_to_int, to_fixed_domain},
    signal_error,
    state::ErrorCode,
    Context, Result, MAX_INPUT_DIMENSIONS,
};

#[derive(Clone)]
pub struct InterpParams<T>
where
    T: Copy,
{
    pub context: Context,
    pub flags: u32,
    pub n_outputs: usize,
    pub n_samples: [usize; MAX_INPUT_DIMENSIONS],
    pub domain: [usize; MAX_INPUT_DIMENSIONS],
    pub opta: [usize; MAX_INPUT_DIMENSIONS],
    pub table: Box<[T]>,
    pub interpolation: InterpFunction,
}

impl<T: Copy> InterpParams<T> {
    pub(crate) fn set_routine(
        context_id: &Context,
        n_inputs: usize,
        n_outputs: usize,
        flags: u32,
    ) -> Result<InterpFunction> {
        let result = (context_id.interpolator)(n_inputs, n_outputs, flags);

        if result.is_ok() {
            return result;
        }

        default_factory(n_inputs, n_outputs, flags)
    }

    pub(crate) fn compute_ex(
        context_id: &Context,
        n_samples: &[usize],
        input_chan: usize,
        output_chan: usize,
        table: &[T],
        flags: u32,
    ) -> Result<InterpParams<T>> {
        // Check for maximum inputs
        if input_chan > MAX_INPUT_DIMENSIONS {
            let msg = format!(
                "Too many input channels ({} channels, max={})",
                input_chan, MAX_INPUT_DIMENSIONS
            );
            signal_error(context_id, Level::Error, ErrorCode::Range, &msg);
            return Err(msg);
        }
        if n_samples.len() != input_chan {
            let msg = "Invalid number of channels";
            return Err(msg.into());
        }

        let mut domain = [0usize; MAX_INPUT_DIMENSIONS];
        for i in 0..input_chan {
            domain[i] = n_samples[i] - 1;
        }

        // Compute factors to apply to each component to index the grid array
        let mut opta = [0usize; MAX_INPUT_DIMENSIONS];
        opta[0] = output_chan;
        for i in 1..(input_chan) {
            opta[i] = opta[i - 1] * n_samples[input_chan - i];
        }

        let interpolation = match Self::set_routine(context_id, input_chan, output_chan, flags) {
            Ok(value) => value,
            Err(_) => {
                let msg = format!(
                    "Unsupported interpolation ({}->{} channels)",
                    input_chan, output_chan
                );
                signal_error(context_id, Level::Error, ErrorCode::UnknownExtension, &msg);
                return Err(msg);
            }
        };

        let mut result = InterpParams {
            context: context_id.clone(),
            flags,
            n_outputs: output_chan,
            n_samples: [0; MAX_INPUT_DIMENSIONS],
            domain,
            opta,
            table: table.into(),
            interpolation,
        };
        result.n_samples[0..(input_chan)].copy_from_slice(n_samples);

        Ok(result)
    }

    pub(crate) fn compute(
        context_id: &Context,
        n_samples: usize,
        input_chan: usize,
        output_chan: usize,
        table: &[T],
        flags: u32,
    ) -> Result<InterpParams<T>> {
        Self::compute_ex(
            context_id,
            &[n_samples; MAX_INPUT_DIMENSIONS],
            input_chan,
            output_chan,
            table,
            flags,
        )
    }
}

#[inline]
fn linear_interp(a: i32, l: i32, h: i32) -> u16 {
    let dif = ((h - l) * a) as u32 + 0x8000;
    let dif = (dif >> 16) + l as u32;

    dif as u16
}

fn lin_lerp_1d<'a>(value: &[u16], output: &'a mut [u16], p: &InterpParams<u16>) -> &'a [u16] {
    let lut_table = p.table.as_ref();

    // if last value or just one point
    if value[0] == 0xFFFF || p.domain[0] == 0 {
        output[0] = lut_table[p.domain[0]]
    } else {
        let val3 = (p.domain[0] * value[0] as usize) as i32;
        let val3 = to_fixed_domain(val3);

        let cell0 = fixed_to_int(val3);
        let rest = fixed_rest_to_int(val3);

        let y0 = lut_table[cell0 as usize];
        let y1 = lut_table[(cell0 + 1) as usize];

        output[0] = linear_interp(rest, y0 as i32, y1 as i32);
    }

    output
}

#[inline]
fn fclamp(v: f32) -> f32 {
    if v.is_nan() {
        return 0.0;
    }
    v.clamp(1e-9, 1.0)
}

fn lin_lerp_1d_f32<'a>(value: &[f32], output: &'a mut [f32], p: &InterpParams<f32>) -> &'a [f32] {
    let lut_table = p.table.as_ref();

    let val2 = fclamp(value[0]);

    // if last value...
    if val2 == 1.0 || p.domain[0] == 0 {
        output[0] = lut_table[p.domain[0]];
    } else {
        let val2 = val2 * p.domain[0] as f32;

        let cell0 = val2.floor() as i32;
        let cell1 = val2.ceil() as i32;

        // rest is 16 LSB bits
        let rest = val2 - cell0 as f32;

        let y0 = lut_table[cell0 as usize];
        let y1 = lut_table[cell1 as usize];

        output[0] = y0 + ((y1 - y0) * rest);
    }

    output
}

fn eval_1_input<'a>(input: &[u16], output: &'a mut [u16], p16: &InterpParams<u16>) -> &'a [u16] {
    let lut_table = &p16.table;

    // if last value...
    if input[0] == 0xFFFF || p16.domain[0] == 0 {
        let y0 = p16.domain[0] * p16.opta[0];

        for out_chan in 0..p16.n_outputs {
            output[out_chan] = lut_table[y0 + out_chan];
        }
    } else {
        let v = input[0] as i32 * p16.domain[0] as i32;
        let fk = to_fixed_domain(v);

        let k0 = fixed_to_int(fk);
        let rk = fixed_rest_to_int(fk) as u16;

        let k1 = k0 + if input[0] != 0xFFFF { 1 } else { 0 };

        let k0 = k0 * p16.opta[0] as i32;
        let k1 = k1 * p16.opta[0] as i32;

        for out_chan in 0..p16.n_outputs {
            output[out_chan] = linear_interp(
                rk as i32,
                lut_table[k0 as usize + out_chan] as i32,
                lut_table[k1 as usize + out_chan] as i32,
            );
        }
    }

    output
}

fn eval_1_input_f32<'a>(value: &[f32], output: &'a mut [f32], p: &InterpParams<f32>) -> &'a [f32] {
    let lut_table = p.table.as_ref();

    let val2 = fclamp(value[0]);

    // if last value...
    if val2 == 1.0 || p.domain[0] == 0 {
        let start = p.domain[0] * p.opta[0];

        for out_chan in 0..p.n_outputs {
            output[out_chan] = lut_table[start + out_chan];
        }
    } else {
        let val2 = val2 * p.domain[0] as f32;

        let cell0 = val2.floor() as i32;
        let cell1 = val2.ceil() as i32;

        let rest = val2 - cell0 as f32;

        let cell0 = cell0 * p.opta[0] as i32;
        let cell1 = cell1 * p.opta[0] as i32;

        for out_chan in 0..p.n_outputs {
            let y0 = lut_table[cell0 as usize + out_chan];
            let y1 = lut_table[cell1 as usize + out_chan];

            output[out_chan] = y0 + ((y1 - y0) * rest);
        }
    }

    output
}

fn bilinear_interp_f32<'a>(
    input: &[f32],
    output: &'a mut [f32],
    p: &InterpParams<f32>,
) -> &'a [f32] {
    let lut_table = p.table.as_ref();

    let total_out = p.n_outputs;
    let px = fclamp(input[0]) * p.domain[0] as f32;
    let py = fclamp(input[1]) * p.domain[1] as f32;

    let x0 = quick_floor(px as f64);
    let fx = px - x0 as f32;
    let y0 = quick_floor(py as f64);
    let fy = py - y0 as f32;

    let x0 = x0 * p.opta[1] as i32;
    let x1 = x0
        + if fclamp(input[0]) >= 1.0 {
            0
        } else {
            p.opta[1] as i32
        };

    let y0 = y0 * p.opta[0] as i32;
    let y1 = y0
        + if fclamp(input[1]) >= 1.0 {
            0
        } else {
            p.opta[0] as i32
        };

    for out_chan in 0..total_out {
        macro_rules! dens {
            ($i:expr, $j:expr) => {
                lut_table[$i as usize + $j as usize + out_chan]
            };
        }
        macro_rules! lerp {
            ($a:expr, $l: expr, $h: expr) => {
                ($l + (($h - $l) * $a)) as f32
            };
        }
        let d00 = dens!(x0, y0);
        let d01 = dens!(x0, y1);
        let d10 = dens!(x1, y0);
        let d11 = dens!(x1, y1);

        let dx0 = lerp!(fx, d00, d10);
        let dx1 = lerp!(fx, d01, d11);

        output[out_chan] = lerp!(fy, dx0, dx1);
    }

    output
}

fn bilinear_interp_16<'a>(input: &[u16], output: &'a mut [u16], p: &InterpParams<u16>) -> &'a [u16] {
    let lut_table = p.table.as_ref();

    let total_out = p.n_outputs;

    let fx = to_fixed_domain(input[0] as i32 * p.domain[0] as i32);
    let x0 = fixed_to_int(fx);
    let rx = fixed_rest_to_int(fx);

    let fy = to_fixed_domain(input[1] as i32 * p.domain[1] as i32);
    let y0 = fixed_to_int(fy);
    let ry = fixed_rest_to_int(fy);

    let x0 = x0 * p.opta[1] as i32;
    let x1 = x0
        + if input[0] == 0xFFFF {
            0
        } else {
            p.opta[1] as i32
        };

    let y0 = y0 * p.opta[0] as i32;
    let y1 = y0
        + if input[1] == 0xFFFF {
            0
        } else {
            p.opta[0] as i32
        };

    for out_chan in 0..total_out {
        macro_rules! dens {
            ($i:expr, $j:expr) => {
                lut_table[$i as usize + $j as usize + out_chan]
            };
        }
        macro_rules! lerp {
            ($a:expr, $l: expr, $h: expr) => {
                ($l as i32 + round_fixed_to_int((($h as i32 - $l as i32) * $a as i32)) as i32)
                    as u16
            };
        }
        let d00 = dens!(x0, y0);
        let d01 = dens!(x0, y1);
        let d10 = dens!(x1, y0);
        let d11 = dens!(x1, y1);

        let dx0 = lerp!(rx, d00, d10);
        let dx1 = lerp!(rx, d01, d11);

        output[out_chan] = lerp!(ry, dx0, dx1);
    }

    output
}

fn trilinear_interp_f32<'a>(
    input: &[f32],
    output: &'a mut [f32],
    p: &InterpParams<f32>,
) -> &'a [f32] {
    let lut_table = p.table.as_ref();

    let total_out = p.n_outputs;

    let px = fclamp(input[0]) * p.domain[0] as f32;
    let py = fclamp(input[1]) * p.domain[1] as f32;
    let pz = fclamp(input[2]) * p.domain[2] as f32;

    // We need full floor functionality here
    let x0 = px.floor() as i32;
    let fx = px - x0 as f32;
    let y0 = py.floor() as i32;
    let fy = py - y0 as f32;
    let z0 = pz.floor() as i32;
    let fz = pz - z0 as f32;

    let x0 = x0 * p.opta[2] as i32;
    let x1 = x0
        + if fclamp(input[0]) >= 1.0 {
            0
        } else {
            p.opta[2] as i32
        };

    let y0 = y0 * p.opta[1] as i32;
    let y1 = y0
        + if fclamp(input[1]) >= 1.0 {
            0
        } else {
            p.opta[1] as i32
        };

    let z0 = z0 * p.opta[0] as i32;
    let z1 = z0
        + if fclamp(input[2]) >= 1.0 {
            0
        } else {
            p.opta[0] as i32
        };

    for out_chan in 0..total_out {
        macro_rules! dens {
            ($i:expr, $j:expr, $k:expr) => {
                lut_table[$i as usize + $j as usize + $k as usize + out_chan]
            };
        }
        macro_rules! lerp {
            ($a:expr, $l: expr, $h: expr) => {
                ($l + (($h - $l) * $a)) as f32
            };
        }
        let d000 = dens!(x0, y0, z0);
        let d001 = dens!(x0, y0, z1);
        let d010 = dens!(x0, y1, z0);
        let d011 = dens!(x0, y1, z1);
        let d100 = dens!(x1, y0, z0);
        let d101 = dens!(x1, y0, z1);
        let d110 = dens!(x1, y1, z0);
        let d111 = dens!(x1, y1, z1);

        let dx00 = lerp!(fx, d000, d100);
        let dx01 = lerp!(fx, d001, d101);
        let dx10 = lerp!(fx, d010, d110);
        let dx11 = lerp!(fx, d011, d111);

        let dxy0 = lerp!(fy, dx00, dx10);
        let dxy1 = lerp!(fy, dx01, dx11);

        output[out_chan] = lerp!(fz, dxy0, dxy1);
    }

    output
}

fn trilinear_interp_16<'a>(
    input: &[u16],
    output: &'a mut [u16],
    p: &InterpParams<u16>,
) -> &'a [u16] {
    let lut_table = p.table.as_ref();

    let total_out = p.n_outputs;

    let fx = to_fixed_domain(input[0] as i32 * p.domain[0] as i32);
    let x0 = fixed_to_int(fx);
    let rx = fixed_rest_to_int(fx);

    let fy = to_fixed_domain(input[1] as i32 * p.domain[1] as i32);
    let y0 = fixed_to_int(fy);
    let ry = fixed_rest_to_int(fy);

    let fz = to_fixed_domain(input[2] as i32 * p.domain[2] as i32);
    let z0 = fixed_to_int(fz);
    let rz = fixed_rest_to_int(fz);

    let x0 = x0 * p.opta[2] as i32;
    let x1 = x0
        + if input[0] == 0xFFFF {
            0
        } else {
            p.opta[2] as i32
        };

    let y0 = y0 * p.opta[1] as i32;
    let y1 = y0
        + if input[1] == 0xFFFF {
            0
        } else {
            p.opta[1] as i32
        };

    let z0 = z0 * p.opta[0] as i32;
    let z1 = z0
        + if input[2] == 0xFFFF {
            0
        } else {
            p.opta[0] as i32
        };

    for out_chan in 0..total_out {
        macro_rules! dens {
            ($i:expr, $j:expr, $k:expr) => {
                lut_table[$i as usize + $j as usize + $k as usize + out_chan]
            };
        }
        macro_rules! lerp {
            ($a:expr, $l: expr, $h: expr) => {
                ($l as i32 + round_fixed_to_int((($h as i32 - $l as i32) * $a as i32)) as i32)
                    as u16
            };
        }
        let d000 = dens!(x0, y0, z0);
        let d001 = dens!(x0, y0, z1);
        let d010 = dens!(x0, y1, z0);
        let d011 = dens!(x0, y1, z1);
        let d100 = dens!(x1, y0, z0);
        let d101 = dens!(x1, y0, z1);
        let d110 = dens!(x1, y1, z0);
        let d111 = dens!(x1, y1, z1);

        let dx00 = lerp!(rx, d000, d100);
        let dx01 = lerp!(rx, d001, d101);
        let dx10 = lerp!(rx, d010, d110);
        let dx11 = lerp!(rx, d011, d111);

        let dxy0 = lerp!(ry, dx00, dx10);
        let dxy1 = lerp!(ry, dx01, dx11);

        output[out_chan] = lerp!(rz, dxy0, dxy1);
    }

    output
}

fn tetrahedral_interp_f32<'a>(
    input: &[f32],
    output: &'a mut [f32],
    p: &InterpParams<f32>,
) -> &'a [f32] {
    let lut_table = p.table.as_ref();

    let total_out = p.n_outputs;

    let px = fclamp(input[0]) * p.domain[0] as f32;
    let py = fclamp(input[1]) * p.domain[1] as f32;
    let pz = fclamp(input[2]) * p.domain[2] as f32;

    // We need full floor functionality here
    let x0 = px.floor() as i32;
    let rx = px - x0 as f32;
    let y0 = py.floor() as i32;
    let ry = py - y0 as f32;
    let z0 = pz.floor() as i32;
    let rz = pz - z0 as f32;

    let x0 = x0 * p.opta[2] as i32;
    let x1 = x0
        + if fclamp(input[0]) >= 1.0 {
            0
        } else {
            p.opta[2] as i32
        };

    let y0 = y0 * p.opta[1] as i32;
    let y1 = y0
        + if fclamp(input[1]) >= 1.0 {
            0
        } else {
            p.opta[1] as i32
        };

    let z0 = z0 * p.opta[0] as i32;
    let z1 = z0
        + if fclamp(input[2]) >= 1.0 {
            0
        } else {
            p.opta[0] as i32
        };

    for out_chan in 0..total_out {
        macro_rules! dens {
            ($i:expr, $j:expr, $k:expr) => {
                lut_table[$i as usize + $j as usize + $k as usize + out_chan]
            };
        }

        let c0 = dens!(x0, y0, z0);

        let (c1, c2, c3) = if rx >= ry {
            if ry >= rz {
                // rx >= ry >= rz
                (
                    dens!(x1, y0, z0) - c0,
                    dens!(x1, y1, z0) - dens!(x1, y0, z0),
                    dens!(x1, y1, z1) - dens!(x1, y1, z0),
                )
            } else if rz >= rx {
                // rz >= rx >= ry
                (
                    dens!(x1, y0, z1) - dens!(x0, y0, z1),
                    dens!(x1, y1, z1) - dens!(x1, y0, z1),
                    dens!(x0, y0, z1) - c0,
                )
            } else {
                // rx >= rz >= ry
                (
                    dens!(x1, y0, z0) - c0,
                    dens!(x1, y1, z1) - dens!(x1, y0, z1),
                    dens!(x1, y0, z1) - dens!(x1, y0, z0),
                )
            }
        } else {
            if rx >= rz {
                // ry >= rx >= rz
                (
                    dens!(x1, y1, z0) - dens!(x0, y1, z0),
                    dens!(x0, y1, z0) - c0,
                    dens!(x1, y1, z1) - dens!(x1, y1, z0),
                )
            } else if ry >= rz {
                // ry >= rz >= rx
                (
                    dens!(x1, y1, z1) - dens!(x0, y1, z1),
                    dens!(x0, y1, z0) - c0,
                    dens!(x0, y1, z1) - dens!(x0, y1, z0),
                )
            } else {
                // rz >= ry >= rx
                (
                    dens!(x1, y1, z1) - dens!(x0, y1, z1),
                    dens!(x0, y1, z1) - dens!(x0, y0, z1),
                    dens!(x0, y0, z1) - c0,
                )
            }
        };

        output[out_chan] = c0 + (c1 * rx) + (c2 * ry) + (c3 * rz);
    }

    output
}

fn tetrahedral_interp_16<'a>(
    input: &[u16],
    output: &'a mut [u16],
    p: &InterpParams<u16>,
) -> &'a [u16] {
    let total_out = p.n_outputs;

    let fx = to_fixed_domain(input[0] as i32 * p.domain[0] as i32);
    let fy = to_fixed_domain(input[1] as i32 * p.domain[1] as i32);
    let fz = to_fixed_domain(input[2] as i32 * p.domain[2] as i32);

    let x0 = fixed_to_int(fx);
    let y0 = fixed_to_int(fy);
    let z0 = fixed_to_int(fz);

    let rx = fixed_rest_to_int(fx);
    let ry = fixed_rest_to_int(fy);
    let rz = fixed_rest_to_int(fz);

    let x0 = x0 * p.opta[2] as i32;
    let mut x1 = x0
        + if input[0] == 0xFFFF {
            0
        } else {
            p.opta[2] as i32
        };

    let y0 = y0 * p.opta[1] as i32;
    let mut y1 = y0
        + if input[1] == 0xFFFF {
            0
        } else {
            p.opta[1] as i32
        };

    let z0 = z0 * p.opta[0] as i32;
    let mut z1 = z0
        + if input[2] == 0xFFFF {
            0
        } else {
            p.opta[0] as i32
        };

    let mut lut_table = &p.table[((x0 + y0 + z0) as usize)..];
    let mut i = 0usize;
    if rx >= ry {
        if ry >= rz {
            // rx >= ry >= rz
            y1 += x1;
            z1 += y1;
            for _ in 0..total_out {
                let c1 = lut_table[x1 as usize] as i32;
                let c2 = lut_table[y1 as usize] as i32;
                let c3 = lut_table[z1 as usize] as i32;
                let c0 = lut_table[0] as i32;
                let c3 = c3 - c2;
                let c2 = c2 - c1;
                let c1 = c1 - c0;
                let rest = (c1 * rx) + (c2 * ry) + (c3 * rz) + 0x8001;
                output[i] = (c0 + ((rest + (rest >> 16)) >> 16)) as u16;
                i = i + 1;
                lut_table = &lut_table[1..];
            }
        } else if rz >= rx {
            // rz >= rx >= ry
            x1 += z1;
            y1 += x1;
            for _ in 0..total_out {
                let c1 = lut_table[x1 as usize] as i32;
                let c2 = lut_table[y1 as usize] as i32;
                let c3 = lut_table[z1 as usize] as i32;
                let c0 = lut_table[0] as i32;
                let c2 = c2 - c1;
                let c1 = c1 - c3;
                let c3 = c3 - c0;
                let rest = (c1 * rx) + (c2 * ry) + (c3 * rz) + 0x8001;
                output[i] = (c0 + ((rest + (rest >> 16)) >> 16)) as u16;
                i = i + 1;
                lut_table = &lut_table[1..];
            }
        } else {
            // rx >= rz >= ry
            z1 += x1;
            y1 += z1;
            for _ in 0..total_out {
                let c1 = lut_table[x1 as usize] as i32;
                let c2 = lut_table[y1 as usize] as i32;
                let c3 = lut_table[z1 as usize] as i32;
                let c0 = lut_table[0] as i32;
                let c2 = c2 - c3;
                let c3 = c3 - c1;
                let c1 = c1 - c0;
                let rest = (c1 * rx) + (c2 * ry) + (c3 * rz) + 0x8001;
                output[i] = (c0 + ((rest + (rest >> 16)) >> 16)) as u16;
                i = i + 1;
                lut_table = &lut_table[1..];
            }
        }
    } else {
        if rx >= rz {
            // ry >= rx >= rz
            x1 += y1;
            z1 += x1;
            for _ in 0..total_out {
                let c1 = lut_table[x1 as usize] as i32;
                let c2 = lut_table[y1 as usize] as i32;
                let c3 = lut_table[z1 as usize] as i32;
                let c0 = lut_table[0] as i32;
                let c3 = c3 - c1;
                let c1 = c1 - c2;
                let c2 = c2 - c0;
                let rest = (c1 * rx) + (c2 * ry) + (c3 * rz) + 0x8001;
                output[i] = (c0 + ((rest + (rest >> 16)) >> 16)) as u16;
                i = i + 1;
                lut_table = &lut_table[1..];
            }
        } else if ry >= rz {
            // ry >= rz >= rx
            z1 += y1;
            x1 += z1;
            for _ in 0..total_out {
                let c1 = lut_table[x1 as usize] as i32;
                let c2 = lut_table[y1 as usize] as i32;
                let c3 = lut_table[z1 as usize] as i32;
                let c0 = lut_table[0] as i32;
                let c1 = c1 - c3;
                let c3 = c3 - c2;
                let c2 = c2 - c0;
                let rest = (c1 * rx) + (c2 * ry) + (c3 * rz) + 0x8001;
                output[i] = (c0 + ((rest + (rest >> 16)) >> 16)) as u16;
                i = i + 1;
                lut_table = &lut_table[1..];
            }
        } else {
            // rz >= ry >= rx
            y1 += z1;
            x1 += y1;
            for _ in 0..total_out {
                let c1 = lut_table[x1 as usize] as i32;
                let c2 = lut_table[y1 as usize] as i32;
                let c3 = lut_table[z1 as usize] as i32;
                let c0 = lut_table[0] as i32;
                let c1 = c1 - c2;
                let c2 = c2 - c3;
                let c3 = c3 - c0;
                let rest = (c1 * rx) + (c2 * ry) + (c3 * rz) + 0x8001;
                output[i] = (c0 + ((rest + (rest >> 16)) >> 16)) as u16;
                i = i + 1;
                lut_table = &lut_table[1..];
            }
        }
    }

    output
}

fn eval_4_inputs<'a>(input: &[u16], output: &'a mut [u16], p16: &InterpParams<u16>) -> &'a [u16] {
    let (mut tmp1, mut tmp2) = ([0u16; MAX_STAGE_CHANNELS], [0u16; MAX_STAGE_CHANNELS]);

    let fk = to_fixed_domain(input[0] as i32 * p16.domain[0] as i32);
    let fx = to_fixed_domain(input[1] as i32 * p16.domain[1] as i32);
    let fy = to_fixed_domain(input[2] as i32 * p16.domain[2] as i32);
    let fz = to_fixed_domain(input[3] as i32 * p16.domain[3] as i32);

    let mut k0 = fixed_to_int(fk);
    let mut x0 = fixed_to_int(fx);
    let mut y0 = fixed_to_int(fy);
    let mut z0 = fixed_to_int(fz);

    let rk = fixed_rest_to_int(fk);
    let rx = fixed_rest_to_int(fx);
    let ry = fixed_rest_to_int(fy);
    let rz = fixed_rest_to_int(fz);

    k0 *= p16.opta[3] as i32;
    let k1 = k0
        + if input[0] == 0xFFFF {
            0
        } else {
            p16.opta[3] as i32
        };

    x0 *= p16.opta[2] as i32;
    let x1 = x0
        + if input[1] == 0xFFFF {
            0
        } else {
            p16.opta[2] as i32
        };

    y0 *= p16.opta[1] as i32;
    let y1 = y0
        + if input[2] == 0xFFFF {
            0
        } else {
            p16.opta[1] as i32
        };

    z0 *= p16.opta[0] as i32;
    let z1 = z0
        + if input[3] == 0xFFFF {
            0
        } else {
            p16.opta[0] as i32
        };

    let mut lut_table = &p16.table[(k0 as usize)..];

    for out_chan in 0..p16.n_outputs {
        macro_rules! dens {
            ($i:expr, $j:expr, $k:expr) => {
                lut_table[$i as usize + $j as usize + $k as usize + out_chan] as i32
            };
        }

        let c0 = dens!(x0, y0, z0);

        let (c1, c2, c3) = if rx >= ry {
            if ry >= rz {
                // rx >= ry >= rz
                (
                    dens!(x1, y0, z0) - c0,
                    dens!(x1, y1, z0) - dens!(x1, y0, z0),
                    dens!(x1, y1, z1) - dens!(x1, y1, z0),
                )
            } else if rz >= rx {
                // rz >= rx >= ry
                (
                    dens!(x1, y0, z1) - dens!(x0, y0, z1),
                    dens!(x1, y1, z1) - dens!(x1, y0, z1),
                    dens!(x0, y0, z1) - c0,
                )
            } else {
                // rx >= rz >= ry
                (
                    dens!(x1, y0, z0) - c0,
                    dens!(x1, y1, z1) - dens!(x1, y0, z1),
                    dens!(x1, y0, z1) - dens!(x1, y0, z0),
                )
            }
        } else {
            if rx >= rz {
                // ry >= rx >= rz
                (
                    dens!(x1, y1, z0) - dens!(x0, y1, z0),
                    dens!(x0, y1, z0) - c0,
                    dens!(x1, y1, z1) - dens!(x1, y1, z0),
                )
            } else if ry >= rz {
                // ry >= rz >= rx
                (
                    dens!(x1, y1, z1) - dens!(x0, y1, z1),
                    dens!(x0, y1, z0) - c0,
                    dens!(x0, y1, z1) - dens!(x0, y1, z0),
                )
            } else {
                // rz >= ry >= rx
                (
                    dens!(x1, y1, z1) - dens!(x0, y1, z1),
                    dens!(x0, y1, z1) - dens!(x0, y0, z1),
                    dens!(x0, y0, z1) - c0,
                )
            }
        };

        let rest = (c1 * rx) + (c2 * ry) + (c3 * rz);
        tmp1[out_chan] = (c0 + round_fixed_to_int(to_fixed_domain(rest))) as u16;
    }

    lut_table = &p16.table[(k1 as usize)..];

    for out_chan in 0..p16.n_outputs {
        macro_rules! dens {
            ($i:expr, $j:expr, $k:expr) => {
                lut_table[$i as usize + $j as usize + $k as usize + out_chan] as i32
            };
        }

        let c0 = dens!(x0, y0, z0);

        let (c1, c2, c3) = if rx >= ry {
            if ry >= rz {
                // rx >= ry >= rz
                (
                    dens!(x1, y0, z0) - c0,
                    dens!(x1, y1, z0) - dens!(x1, y0, z0),
                    dens!(x1, y1, z1) - dens!(x1, y1, z0),
                )
            } else if rz >= rx {
                // rz >= rx >= ry
                (
                    dens!(x1, y0, z1) - dens!(x0, y0, z1),
                    dens!(x1, y1, z1) - dens!(x1, y0, z1),
                    dens!(x0, y0, z1) - c0,
                )
            } else {
                // rx >= rz >= ry
                (
                    dens!(x1, y0, z0) - c0,
                    dens!(x1, y1, z1) - dens!(x1, y0, z1),
                    dens!(x1, y0, z1) - dens!(x1, y0, z0),
                )
            }
        } else {
            if rx >= rz {
                // ry >= rx >= rz
                (
                    dens!(x1, y1, z0) - dens!(x0, y1, z0),
                    dens!(x0, y1, z0) - c0,
                    dens!(x1, y1, z1) - dens!(x1, y1, z0),
                )
            } else if ry >= rz {
                // ry >= rz >= rx
                (
                    dens!(x1, y1, z1) - dens!(x0, y1, z1),
                    dens!(x0, y1, z0) - c0,
                    dens!(x0, y1, z1) - dens!(x0, y1, z0),
                )
            } else {
                // rz >= ry >= rx
                (
                    dens!(x1, y1, z1) - dens!(x0, y1, z1),
                    dens!(x0, y1, z1) - dens!(x0, y0, z1),
                    dens!(x0, y0, z1) - c0,
                )
            }
        };

        let rest = (c1 * rx) + (c2 * ry) + (c3 * rz);
        tmp2[out_chan] = (c0 + round_fixed_to_int(to_fixed_domain(rest))) as u16;
    }

    for i in 0..p16.n_outputs {
        output[i] = linear_interp(rk, tmp1[i] as i32, tmp2[i] as i32);
    }

    output
}

fn eval_4_inputs_f32<'a>(input: &[f32], output: &'a mut [f32], p: &InterpParams<f32>) -> &'a [f32] {
    let (mut tmp1, mut tmp2) = ([0f32; MAX_STAGE_CHANNELS], [0f32; MAX_STAGE_CHANNELS]);
    let lut_table = p.table.as_ref();

    let pk = fclamp(input[0]) * p.domain[0] as f32;
    let k0 = quick_floor(pk as f64);
    let rest = pk - k0 as f32;

    let k0 = k0 *p.opta[3] as i32;
    let k1 = k0
        + if fclamp(input[0]) >= 1.0 {
            0
        } else {
            p.opta[2] as i32
        };

    let mut p1 = p.clone();
    p1.domain[0..3].copy_from_slice(&p.domain[1..4]);

    let t = &lut_table[(k0 as usize)..];
    p1.table = t.into();

    tetrahedral_interp_f32(&input[1..], &mut tmp1, &p1);

    let t = &lut_table[(k1 as usize)..];
    p1.table = t.into();

    tetrahedral_interp_f32(&input[1..], &mut tmp2, &p1);
    
    for i in 0..p.n_outputs {
        let y0 = tmp1[i];
        let y1 = tmp2[i];

        output[i] = y0 + (y1 - y0) * rest;
    }

    output
}

macro_rules! eval_fns {
    ($n:literal, $nm:literal) => {
        paste::paste! {
            fn [<eval_ $n _inputs>]<'a>(input: &[u16], output: &'a mut [u16], p16: &InterpParams<u16>) -> &'a [u16] {
                let (mut tmp1, mut tmp2) = ([0u16; MAX_STAGE_CHANNELS], [0u16; MAX_STAGE_CHANNELS]);
                let lut_table = p16.table.as_ref();
            
                let fk = to_fixed_domain(input[0] as i32 * p16.domain[0] as i32);
                let k0 = fixed_to_int(fk);
                let rk = fixed_rest_to_int(fk);
            
                let k0 = p16.opta[$nm] as i32 * k0;
                let k1 = p16.opta[3] as i32 * (k0
                    + if input[0] == 0xFFFF {
                        0
                    } else {
                        1
                    });
                
                let mut p1 = p16.clone();
                p1.domain[0..$nm].copy_from_slice(&p16.domain[1..$n]);

                let t = &lut_table[(k0 as usize)..];
                p1.table = t.into();
                
                [<eval_ $nm _inputs>](&input[1..], &mut tmp1, &p1);

                let t = &lut_table[(k1 as usize)..];
                p1.table = t.into();
                
                [<eval_ $nm _inputs>](&input[1..], &mut tmp2, &p1);
            
                for i in 0..p16.n_outputs {
                    output[i] = linear_interp(rk, tmp1[i] as i32, tmp2[i] as i32);
                }
            
                output
            }
            
            fn [<eval_ $n _inputs_f32>]<'a>(input: &[f32], output: &'a mut [f32], p: &InterpParams<f32>) -> &'a [f32] {
                let (mut tmp1, mut tmp2) = ([0f32; MAX_STAGE_CHANNELS], [0f32; MAX_STAGE_CHANNELS]);
                let lut_table = p.table.as_ref();
            
                let pk = fclamp(input[0]) * p.domain[0] as f32;
                let k0 = quick_floor(pk as f64);
                let rest = pk - k0 as f32;
            
                let k0 = k0 *p.opta[$nm] as i32;
                let k1 = k0
                    + if fclamp(input[0]) >= 1.0 {
                        0
                    } else {
                        p.opta[$nm] as i32
                    };
            
                let mut p1 = p.clone();
                p1.domain[0..$nm].copy_from_slice(&p.domain[1..$n]);
            
                let t = &lut_table[(k0 as usize)..];
                p1.table = t.into();
            
                [<eval_ $nm _inputs_f32>](&input[1..], &mut tmp1, &p1);
            
                let t = &lut_table[(k1 as usize)..];
                p1.table = t.into();
            
                [<eval_ $nm _inputs_f32>](&input[1..], &mut tmp2, &p1);
                
                for i in 0..p.n_outputs {
                    let y0 = tmp1[i];
                    let y1 = tmp2[i];
            
                    output[i] = y0 + (y1 - y0) * rest;
                }
            
                output
            }
        }
    };
}
eval_fns!(5, 4);
eval_fns!(6, 5);
eval_fns!(7, 6);
eval_fns!(8, 7);
eval_fns!(9, 8);
eval_fns!(10, 9);
eval_fns!(11, 10);
eval_fns!(12, 11);
eval_fns!(13, 12);
eval_fns!(14, 13);
eval_fns!(15, 14);

pub(crate) fn default_factory(
    n_inputs: usize,
    n_outputs: usize,
    flags: u32,
) -> Result<InterpFunction> {
    let is_float = (flags & lerp_flag::FLOAT) != 0;
    let is_trilinear = (flags & lerp_flag::TRILINEAR) != 0;

    // Safety check
    if n_inputs >= 4 && n_outputs >= MAX_STAGE_CHANNELS {
        return Err("Invalid interp setup".into());
    }

    match n_inputs {
        1 => {      // Gray Lut / linear
            if n_outputs == 1 {
                if is_float {
                    Ok(InterpFunction::F32(lin_lerp_1d_f32))
                } else {
                    Ok(InterpFunction::U16(lin_lerp_1d))
                }
            } else {
                if is_float {
                    Ok(InterpFunction::F32(eval_1_input_f32))
                } else {
                    Ok(InterpFunction::U16(eval_1_input))
                }
            }
        },
        2 => {      // Duotone
            if is_float {
                Ok(InterpFunction::F32(bilinear_interp_f32))
            } else {
                Ok(InterpFunction::U16(bilinear_interp_16))
            }
        },
        3 => {      // RGB et al
            if is_trilinear {
                if is_float {
                    Ok(InterpFunction::F32(trilinear_interp_f32))
                } else {
                    Ok(InterpFunction::U16(trilinear_interp_16))
                }
            } else {
                if is_float {
                    Ok(InterpFunction::F32(tetrahedral_interp_f32))
                } else {
                    Ok(InterpFunction::U16(tetrahedral_interp_16))
                }
            }
        },
        4 => {      // CMYK lut
            if is_float {
                Ok(InterpFunction::F32(eval_4_inputs_f32))
            } else {
                Ok(InterpFunction::U16(eval_4_inputs))
            }
        },
        5 => {
            if is_float {
                Ok(InterpFunction::F32(eval_5_inputs_f32))
            } else {
                Ok(InterpFunction::U16(eval_5_inputs))
            }
        },
        6 => {
            if is_float {
                Ok(InterpFunction::F32(eval_6_inputs_f32))
            } else {
                Ok(InterpFunction::U16(eval_6_inputs))
            }
        },
        7 => {
            if is_float {
                Ok(InterpFunction::F32(eval_7_inputs_f32))
            } else {
                Ok(InterpFunction::U16(eval_7_inputs))
            }
        },
        8 => {
            if is_float {
                Ok(InterpFunction::F32(eval_8_inputs_f32))
            } else {
                Ok(InterpFunction::U16(eval_8_inputs))
            }
        },
        9 => {
            if is_float {
                Ok(InterpFunction::F32(eval_9_inputs_f32))
            } else {
                Ok(InterpFunction::U16(eval_9_inputs))
            }
        },
        10 => {
            if is_float {
                Ok(InterpFunction::F32(eval_10_inputs_f32))
            } else {
                Ok(InterpFunction::U16(eval_10_inputs))
            }
        },
        11 => {
            if is_float {
                Ok(InterpFunction::F32(eval_11_inputs_f32))
            } else {
                Ok(InterpFunction::U16(eval_11_inputs))
            }
        },
        12 => {
            if is_float {
                Ok(InterpFunction::F32(eval_12_inputs_f32))
            } else {
                Ok(InterpFunction::U16(eval_12_inputs))
            }
        },
        13 => {
            if is_float {
                Ok(InterpFunction::F32(eval_13_inputs_f32))
            } else {
                Ok(InterpFunction::U16(eval_13_inputs))
            }
        },
        14 => {
            if is_float {
                Ok(InterpFunction::F32(eval_14_inputs_f32))
            } else {
                Ok(InterpFunction::U16(eval_14_inputs))
            }
        },
        15 => {
            if is_float {
                Ok(InterpFunction::F32(eval_15_inputs_f32))
            } else {
                Ok(InterpFunction::U16(eval_15_inputs))
            }
        },
        _ => Err("Invalid number of channels".into())
    }
}

pub mod lerp_flag {
    pub const U16_BITS: u32 = 0x0000;
    pub const FLOAT: u32 = 0x0001;
    pub const TRILINEAR: u32 = 0x0100;
}

pub type InterpFn<T> = for<'a> fn(Input: &'a [T], Output: &'a mut [T], p: &'a InterpParams<T>) -> &'a [T];

#[derive(Clone)]
pub enum InterpFunction {
    F32(InterpFn<f32>),
    U16(InterpFn<u16>),
}

impl InterpFunction {
    pub const fn is_f32(&self) -> bool {
        matches!(*self, Self::F32(_))
    }
    pub const fn is_u16(&self) -> bool {
        matches!(*self, Self::U16(_))
    }
    pub fn is_f32_and(self, f: impl FnOnce(InterpFn<f32>) -> bool) -> bool {
        match self {
            Self::U16(_) => false,
            Self::F32(x) => f(x),
        }
    }
    pub fn is_u16_and(self, f: impl FnOnce(InterpFn<u16>) -> bool) -> bool {
        match self {
            Self::U16(x) => f(x),
            Self::F32(_) => false,
        }
    }
}

impl From<InterpFn<u16>> for InterpFunction {
    fn from(val: InterpFn<u16>) -> InterpFunction {
        InterpFunction::U16(val)
    }
}

impl From<InterpFn<f32>> for InterpFunction {
    fn from(val: InterpFn<f32>) -> InterpFunction {
        InterpFunction::F32(val)
    }
}
