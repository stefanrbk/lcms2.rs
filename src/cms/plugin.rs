// // Vectors
pub use crate::types::Vec3 as VEC3;

// // 3x3 Matrix
pub use crate::types::Mat3 as MAT3;

// CMSAPI void               CMSEXPORT _cmsVEC3init(cmsVEC3* r, cmsFloat64Number x, cmsFloat64Number y, cmsFloat64Number z);
// CMSAPI void               CMSEXPORT _cmsVEC3minus(cmsVEC3* r, const cmsVEC3* a, const cmsVEC3* b);
// CMSAPI void               CMSEXPORT _cmsVEC3cross(cmsVEC3* r, const cmsVEC3* u, const cmsVEC3* v);
// CMSAPI cmsFloat64Number   CMSEXPORT _cmsVEC3dot(const cmsVEC3* u, const cmsVEC3* v);
// CMSAPI cmsFloat64Number   CMSEXPORT _cmsVEC3length(const cmsVEC3* a);
// CMSAPI cmsFloat64Number   CMSEXPORT _cmsVEC3distance(const cmsVEC3* a, const cmsVEC3* b);

// CMSAPI void               CMSEXPORT _cmsMAT3identity(cmsMAT3* a);
// CMSAPI cmsBool            CMSEXPORT _cmsMAT3isIdentity(const cmsMAT3* a);
// CMSAPI void               CMSEXPORT _cmsMAT3per(cmsMAT3* r, const cmsMAT3* a, const cmsMAT3* b);
// CMSAPI cmsBool            CMSEXPORT _cmsMAT3inverse(const cmsMAT3* a, cmsMAT3* b);
// CMSAPI cmsBool            CMSEXPORT _cmsMAT3solve(cmsVEC3* x, cmsMAT3* a, cmsVEC3* b);
// CMSAPI void               CMSEXPORT _cmsMAT3eval(cmsVEC3* r, const cmsMAT3* a, const cmsVEC3* v);

// MD5 low level  -------------------------------------------------------------------------------------

// CMSAPI cmsHANDLE          CMSEXPORT cmsMD5alloc(cmsContext ContextID);
// CMSAPI void               CMSEXPORT cmsMD5add(cmsHANDLE Handle, const cmsUInt8Number* buf, cmsUInt32Number len);
// CMSAPI void               CMSEXPORT cmsMD5finish(cmsProfileID* ProfileID, cmsHANDLE Handle);

// Error logging  -------------------------------------------------------------------------------------

// CMSAPI void               CMSEXPORT  cmsSignalError(cmsContext ContextID, cmsUInt32Number ErrorCode, const char *ErrorText, ...);

// Memory management ----------------------------------------------------------------------------------

// CMSAPI void*              CMSEXPORT _cmsMalloc(cmsContext ContextID, cmsUInt32Number size);
// CMSAPI void*              CMSEXPORT _cmsMallocZero(cmsContext ContextID, cmsUInt32Number size);
// CMSAPI void*              CMSEXPORT _cmsCalloc(cmsContext ContextID, cmsUInt32Number num, cmsUInt32Number size);
// CMSAPI void*              CMSEXPORT _cmsRealloc(cmsContext ContextID, void* Ptr, cmsUInt32Number NewSize);
// CMSAPI void               CMSEXPORT _cmsFree(cmsContext ContextID, void* Ptr);
// CMSAPI void*              CMSEXPORT _cmsDupMem(cmsContext ContextID, const void* Org, cmsUInt32Number size);

// I/O handler ----------------------------------------------------------------------------------

pub use crate::io::IoHandler as IOHANDLER;

// // Endianness adjust functions
// CMSAPI cmsUInt16Number   CMSEXPORT  _cmsAdjustEndianess16(cmsUInt16Number Word);
// CMSAPI cmsUInt32Number   CMSEXPORT  _cmsAdjustEndianess32(cmsUInt32Number Value);
// CMSAPI void              CMSEXPORT  _cmsAdjustEndianess64(cmsUInt64Number* Result, cmsUInt64Number* QWord);

// // Helper IO functions
// CMSAPI cmsBool           CMSEXPORT  _cmsReadUInt8Number(cmsIOHANDLER* io,  cmsUInt8Number* n);
// CMSAPI cmsBool           CMSEXPORT  _cmsReadUInt16Number(cmsIOHANDLER* io, cmsUInt16Number* n);
// CMSAPI cmsBool           CMSEXPORT  _cmsReadUInt32Number(cmsIOHANDLER* io, cmsUInt32Number* n);
// CMSAPI cmsBool           CMSEXPORT  _cmsReadFloat32Number(cmsIOHANDLER* io, cmsFloat32Number* n);
// CMSAPI cmsBool           CMSEXPORT  _cmsReadUInt64Number(cmsIOHANDLER* io, cmsUInt64Number* n);
// CMSAPI cmsBool           CMSEXPORT  _cmsRead15Fixed16Number(cmsIOHANDLER* io, cmsFloat64Number* n);
// CMSAPI cmsBool           CMSEXPORT  _cmsReadXYZNumber(cmsIOHANDLER* io, cmsCIEXYZ* XYZ);
// CMSAPI cmsBool           CMSEXPORT  _cmsReadUInt16Array(cmsIOHANDLER* io, cmsUInt32Number n, cmsUInt16Number* Array);

// CMSAPI cmsBool           CMSEXPORT  _cmsWriteUInt8Number(cmsIOHANDLER* io, cmsUInt8Number n);
// CMSAPI cmsBool           CMSEXPORT  _cmsWriteUInt16Number(cmsIOHANDLER* io, cmsUInt16Number n);
// CMSAPI cmsBool           CMSEXPORT  _cmsWriteUInt32Number(cmsIOHANDLER* io, cmsUInt32Number n);
// CMSAPI cmsBool           CMSEXPORT  _cmsWriteFloat32Number(cmsIOHANDLER* io, cmsFloat32Number n);
// CMSAPI cmsBool           CMSEXPORT  _cmsWriteUInt64Number(cmsIOHANDLER* io, cmsUInt64Number* n);
// CMSAPI cmsBool           CMSEXPORT  _cmsWrite15Fixed16Number(cmsIOHANDLER* io, cmsFloat64Number n);
// CMSAPI cmsBool           CMSEXPORT  _cmsWriteXYZNumber(cmsIOHANDLER* io, const cmsCIEXYZ* XYZ);
// CMSAPI cmsBool           CMSEXPORT  _cmsWriteUInt16Array(cmsIOHANDLER* io, cmsUInt32Number n, const cmsUInt16Number* Array);

// // ICC base tag
// typedef struct {
//     cmsTagTypeSignature  sig;
//     cmsInt8Number        reserved[4];

// } _cmsTagBase;

// // Type base helper functions
// CMSAPI cmsTagTypeSignature  CMSEXPORT _cmsReadTypeBase(cmsIOHANDLER* io);
// CMSAPI cmsBool              CMSEXPORT _cmsWriteTypeBase(cmsIOHANDLER* io, cmsTagTypeSignature sig);

// // Alignment functions
// CMSAPI cmsBool             CMSEXPORT _cmsReadAlignment(cmsIOHANDLER* io);
// CMSAPI cmsBool             CMSEXPORT _cmsWriteAlignment(cmsIOHANDLER* io);

// // To deal with text streams. 2K at most
// CMSAPI cmsBool             CMSEXPORT _cmsIOPrintf(cmsIOHANDLER* io, const char* frm, ...);

// // Fixed point helper functions
// CMSAPI cmsFloat64Number    CMSEXPORT _cms8Fixed8toDouble(cmsUInt16Number fixed8);
// CMSAPI cmsUInt16Number     CMSEXPORT _cmsDoubleTo8Fixed8(cmsFloat64Number val);

// CMSAPI cmsFloat64Number    CMSEXPORT _cms15Fixed16toDouble(cmsS15Fixed16Number fix32);
// CMSAPI cmsS15Fixed16Number CMSEXPORT _cmsDoubleTo15Fixed16(cmsFloat64Number v);

// // Date/time helper functions
// CMSAPI void                CMSEXPORT _cmsEncodeDateTimeNumber(cmsDateTimeNumber *Dest, const struct tm *Source);
// CMSAPI void                CMSEXPORT _cmsDecodeDateTimeNumber(const cmsDateTimeNumber *Source, struct tm *Dest);

//----------------------------------------------------------------------------------------------------------

// Shared callbacks for user data
pub use crate::plugin::{DupUserDataFn, FreeUserDataFn};

// //----------------------------------------------------------------------------------------------------------

// // Plug-in foundation
pub use crate::sig::plugin::MAGIC_NUMBER as PLUGIN_MAGIC_NUMBER;

pub use crate::sig::plugin::FORMATTERS as PLUGIN_FORMATTERS_SIG;
pub use crate::sig::plugin::INTERPOLATION as PLUGIN_INTERPOLATION_SIG;
pub use crate::sig::plugin::MEM_HANDLER as PLUGIN_MEM_HANDLER_SIG;
pub use crate::sig::plugin::MULTI_PROCESS_ELEMENT as PLUGIN_MULTI_PROCESS_ELEMENT_SIG;
pub use crate::sig::plugin::MUTEX as PLUGIN_MUTEX_SIG;
pub use crate::sig::plugin::OPTIMIZATION as PLUGIN_OPTIMIZATION_SIG;
pub use crate::sig::plugin::PARALLELIZATION as PLUGIN_PARALELLIZATION_SIG;
pub use crate::sig::plugin::PARAMETRIC_CURVE as PLUGIN_PARAMETRIC_CURVE_SIG;
pub use crate::sig::plugin::RENDERING_INTENT as PLUGIN_RENDERING_INTENT_SIG;
pub use crate::sig::plugin::TAG as PLUGIN_TAG_SIG;
pub use crate::sig::plugin::TAG_TYPE as PLUGIN_TAG_TYPE_SIG;
pub use crate::sig::plugin::TRANSFORM as PLUGIN_TRANSFORM_SIG;

pub use crate::plugin::Base as PluginBase;

// Maximum number of types in a plugin array
pub use crate::MAX_TYPES_IN_PLUGIN as MAX_TYPES_IN_LCMS_PLUGIN;

//----------------------------------------------------------------------------------------------------------

// Memory handler. Each new plug-in type replaces current behaviour

// typedef void* (* _cmsMallocFnPtrType)(cmsContext ContextID, cmsUInt32Number size);
// typedef void  (* _cmsFreeFnPtrType)(cmsContext ContextID, void *Ptr);
// typedef void* (* _cmsReallocFnPtrType)(cmsContext ContextID, void* Ptr, cmsUInt32Number NewSize);

// typedef void* (* _cmsMalloZerocFnPtrType)(cmsContext ContextID, cmsUInt32Number size);
// typedef void* (* _cmsCallocFnPtrType)(cmsContext ContextID, cmsUInt32Number num, cmsUInt32Number size);
// typedef void* (* _cmsDupFnPtrType)(cmsContext ContextID, const void* Org, cmsUInt32Number size);

// typedef struct {

//         cmsPluginBase base;

//         // Required
//         _cmsMallocFnPtrType  MallocPtr;
//         _cmsFreeFnPtrType    FreePtr;
//         _cmsReallocFnPtrType ReallocPtr;

//         // Optional
//        _cmsMalloZerocFnPtrType MallocZeroPtr;
//        _cmsCallocFnPtrType     CallocPtr;
//        _cmsDupFnPtrType        DupPtr;

// } cmsPluginMemHandler;

// ------------------------------------------------------------------------------------------------------------------

// Interpolation. 16 bits and floating point versions.
// struct _cms_interp_struc;

// Interpolation callbacks

// 16 bits forward interpolation. This function performs precision-limited linear interpolation
// and is supposed to be quite fast. Implementation may be tetrahedral or trilinear, and plug-ins may
// choose to implement any other interpolation algorithm.
pub type InterpFn16 = crate::types::InterpFn<u16>;

// Floating point forward interpolation. Full precision interpolation using floats. This is not a
// time critical function. Implementation may be tetrahedral or trilinear, and plug-ins may
// choose to implement any other interpolation algorithm.
pub type InterpFnFloat = crate::types::InterpFn<f32>;

// This type holds a pointer to an interpolator that can be either 16 bits or float
pub use crate::types::InterpFunction;

// Flags for interpolator selection
pub use crate::types::lerp_flag::FLOAT as LERP_FLAGS_FLOAT;
pub use crate::types::lerp_flag::TRILINEAR as LERP_FLAGS_TRILINEAR;
pub use crate::types::lerp_flag::U16_BITS as LERP_FLAGS_16BITS;

pub use crate::MAX_INPUT_DIMENSIONS;

// Used on all interpolations. Supplied by lcms2 when calling the interpolation function
pub use crate::types::InterpParams;

// Interpolators factory
pub use crate::plugin::InterpFnFactory;

// // The plug-in
pub use crate::plugin::Interpolation as PluginInterpolation;

//----------------------------------------------------------------------------------------------------------

// Parametric curves. A negative type means same function but analytically inverted. Max. number of params is 10

// Evaluator callback for user-supplied parametric curves. May implement more than one type
pub use crate::plugin::ParametricCurveEvaluator;

// Plug-in may implement an arbitrary number of parametric curves
pub use crate::plugin::ParametricCurve as PluginParametricCurves;

//----------------------------------------------------------------------------------------------------------

// Formatters. This plug-in adds new handlers, replacing them if they already exist. Formatters dealing with
// cmsFloat32Number (bps = 4) or double (bps = 0) types are requested via FormatterFloat callback. Others come across
// Formatter16 callback

pub use crate::plugin::{Formatter16In, Formatter16Out, FormatterFloatIn, FormatterFloatOut};

// This type holds a pointer to a formatter that can be either 16 bits or cmsFloat32Number
pub use crate::plugin::{FormatterIn, FormatterOut};

pub use crate::plugin::pack_flags::FLOAT as PACK_FLAGS_FLOAT;
pub use crate::plugin::pack_flags::U16_BITS as PACK_FLAGS_16BITS;

pub use crate::plugin::{FormatterFactoryIn, FormatterFactoryOut};

pub use crate::plugin::Formatter as PluginFormatters;

//----------------------------------------------------------------------------------------------------------

// Tag type handler. Each type is free to return anything it wants, and it is up to the caller to
// know in advance what is the type contained in the tag.
pub use crate::plugin::TagTypeHandler;

// Each plug-in implements a single type
pub use crate::plugin::TagType as PluginTagType;

//----------------------------------------------------------------------------------------------------------

// This is the tag plugin, which identifies tags. For writing, a pointer to function is provided.
// This function should return the desired type for this tag, given the version of profile
// and the data being serialized.
pub use crate::plugin::TagDescriptor;

// Plug-in implements a single tag
pub use crate::plugin::Tag as PluginTag;

//----------------------------------------------------------------------------------------------------------

// Custom intents. This function should join all profiles specified in the array in
// a single LUT. Any custom intent in the chain redirects to custom function. If more than
// one custom intent is found, the one located first is invoked. Usually users should use only one
// custom intent, so mixing custom intents in same multiprofile transform is not supported.

pub use crate::plugin::IntentFn;

// Each plug-in defines a single intent number.
pub use crate::plugin::RenderingIntent as PluginRenderingIntent;

// The default ICC intents (perceptual, saturation, rel.col and abs.col)
// CMSAPI cmsPipeline*  CMSEXPORT _cmsDefaultICCintents(cmsContext       ContextID,
//                                                      cmsUInt32Number  nProfiles,
//                                                      cmsUInt32Number  Intents[],
//                                                      cmsHPROFILE      hProfiles[],
//                                                      cmsBool          BPC[],
//                                                      cmsFloat64Number AdaptationStates[],
//                                                      cmsUInt32Number  dwFlags);

//----------------------------------------------------------------------------------------------------------

// Pipelines, Multi Process Elements.

pub use crate::types::{StageDupElemFn, StageEvalFn, StageFreeElemFn};

// // This function allocates a generic MPE
// CMSAPI cmsStage* CMSEXPORT _cmsStageAllocPlaceholder(cmsContext ContextID,
//                                 cmsStageSignature     Type,
//                                 cmsUInt32Number       InputChannels,
//                                 cmsUInt32Number       OutputChannels,
//                                 _cmsStageEvalFn       EvalPtr,            // Points to fn that evaluates the element (always in floating point)
//                                 _cmsStageDupElemFn    DupElemPtr,         // Points to a fn that duplicates the stage
//                                 _cmsStageFreeElemFn   FreePtr,            // Points to a fn that sets the element free
//                                 void*                 Data);              // A generic pointer to whatever memory needed by the element

pub use crate::plugin::MultiProcessElement as PluginMultiProcessElement;

// Data kept in "Element" member of cmsStage

// Curves
pub use crate::types::ToneCurvesStageData as StageToneCurvesData;

// Matrix
pub use crate::types::MatrixStageData as StageMatrixData;

// CLUT
pub use crate::types::CLutStageData as StageCLutData;
pub use crate::types::CLutStageDataTab as StageCLutDataTab;

//----------------------------------------------------------------------------------------------------------
// Optimization. Using this plug-in, additional optimization strategies may be implemented.
// The function should return TRUE if any optimization is done on the LUT, this terminates
// the optimization  search. Or FALSE if it is unable to optimize and want to give a chance
// to the rest of optimizers.

pub use crate::plugin::OptimizationFn as OPToptimizeFn;

// Pipeline Evaluator (in 16 bits)
pub use crate::types::PipelineEval16Fn;

// Pipeline Evaluator (in floating point)
pub use crate::types::PipelineEvalFloatFn;

// This function may be used to set the optional evaluator and a block of private data. If private data is being used, an optional
// duplicator and free functions should also be specified in order to duplicate the LUT construct. Use NULL to inhibit such functionality.

// CMSAPI void CMSEXPORT _cmsPipelineSetOptimizationParameters(cmsPipeline* Lut,
//                                                _cmsPipelineEval16Fn Eval16,
//                                                void* PrivateData,
//                                                _cmsFreeUserDataFn FreePrivateDataFn,
//                                                _cmsDupUserDataFn DupPrivateDataFn);

pub use crate::plugin::Optimization as PluginOptimization;

//----------------------------------------------------------------------------------------------------------
// Full xform

pub use crate::types::{Stride, Transform2Factory, Transform2Fn, TransformFactory, TransformFn};

// Retrieve user data as specified by the factory
// CMSAPI void   CMSEXPORT _cmsSetTransformUserData(struct _cmstransform_struct *CMMcargo, void* ptr, _cmsFreeUserDataFn FreePrivateDataFn);
// CMSAPI void * CMSEXPORT _cmsGetTransformUserData(struct _cmstransform_struct *CMMcargo);

// Retrieve formatters
// CMSAPI void   CMSEXPORT _cmsGetTransformFormatters16   (struct _cmstransform_struct *CMMcargo, cmsFormatter16* FromInput, cmsFormatter16* ToOutput);
// CMSAPI void   CMSEXPORT _cmsGetTransformFormattersFloat(struct _cmstransform_struct *CMMcargo, cmsFormatterFloat* FromInput, cmsFormatterFloat* ToOutput);

// Retrieve original flags
// CMSAPI cmsUInt32Number CMSEXPORT _cmsGetTransformFlags(struct _cmstransform_struct* CMMcargo);

pub use crate::plugin::Transform as PluginTransform;

//----------------------------------------------------------------------------------------------------------
// Mutex

pub use crate::plugin::CreateMutexFn as CreateMutexFnPtrType;
pub use crate::plugin::DestroyMutexFn as DestroyMutexFnPtrType;
pub use crate::plugin::LockMutexFn as LockMutexFnPtrType;
pub use crate::plugin::UnlockMutexFn as UnlockMutexFnPtrType;

pub use crate::plugin::Mutex as PluginMutex;

pub use crate::plugin::{IMutex, MutexGuard};

// CMSAPI void*   CMSEXPORT _cmsCreateMutex(cmsContext ContextID);
// CMSAPI void    CMSEXPORT _cmsDestroyMutex(cmsContext ContextID, void* mtx);
// CMSAPI cmsBool CMSEXPORT _cmsLockMutex(cmsContext ContextID, void* mtx);
// CMSAPI void    CMSEXPORT _cmsUnlockMutex(cmsContext ContextID, void* mtx);

//----------------------------------------------------------------------------------------------------------
// Parallelization

// CMSAPI _cmsTransform2Fn CMSEXPORT _cmsGetTransformWorker(struct _cmstransform_struct* CMMcargo);
// CMSAPI cmsInt32Number   CMSEXPORT _cmsGetTransformMaxWorkers(struct _cmstransform_struct* CMMcargo);
// CMSAPI cmsUInt32Number  CMSEXPORT _cmsGetTransformWorkerFlags(struct _cmstransform_struct* CMMcargo);

// Let's plug-in to guess the best number of workers
pub use crate::plugin::GUESS_MAX_WORKERS;

pub use crate::plugin::Parallelization as PluginParalellization;
