pub type ICatalog = *mut ::core::ffi::c_void;
pub type IComponentUtil = *mut ::core::ffi::c_void;
pub type IPackageUtil = *mut ::core::ffi::c_void;
pub type IRemoteComponentUtil = *mut ::core::ffi::c_void;
pub type IRoleAssociationUtil = *mut ::core::ffi::c_void;
pub const Catalog: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169537, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const CatalogCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169539, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const CatalogObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169538, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const ComponentUtil: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169540, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const PackageUtil: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169541, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const RemoteComponentUtil: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169542, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
pub const RoleAssociationUtil: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1857169543, data2: 35353, data3: 4560, data4: [129, 182, 0, 160, 201, 35, 28, 41] };
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub type MTSAdminErrorCodes = i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrObjectErrors: MTSAdminErrorCodes = -2146368511i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrObjectInvalid: MTSAdminErrorCodes = -2146368510i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrKeyMissing: MTSAdminErrorCodes = -2146368509i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrAlreadyInstalled: MTSAdminErrorCodes = -2146368508i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrDownloadFailed: MTSAdminErrorCodes = -2146368507i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPDFWriteFail: MTSAdminErrorCodes = -2146368505i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPDFReadFail: MTSAdminErrorCodes = -2146368504i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPDFVersion: MTSAdminErrorCodes = -2146368503i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCoReqCompInstalled: MTSAdminErrorCodes = -2146368496i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadPath: MTSAdminErrorCodes = -2146368502i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPackageExists: MTSAdminErrorCodes = -2146368501i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrRoleExists: MTSAdminErrorCodes = -2146368500i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCantCopyFile: MTSAdminErrorCodes = -2146368499i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoTypeLib: MTSAdminErrorCodes = -2146368498i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoUser: MTSAdminErrorCodes = -2146368497i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrInvalidUserids: MTSAdminErrorCodes = -2146368496i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryCLSID: MTSAdminErrorCodes = -2146368495i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadRegistryProgID: MTSAdminErrorCodes = -2146368494i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrAuthenticationLevel: MTSAdminErrorCodes = -2146368493i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrUserPasswdNotValid: MTSAdminErrorCodes = -2146368492i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryRead: MTSAdminErrorCodes = -2146368491i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryWrite: MTSAdminErrorCodes = -2146368490i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoRegistryRepair: MTSAdminErrorCodes = -2146368489i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCLSIDOrIIDMismatch: MTSAdminErrorCodes = -2146368488i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrRemoteInterface: MTSAdminErrorCodes = -2146368487i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrDllRegisterServer: MTSAdminErrorCodes = -2146368486i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoServerShare: MTSAdminErrorCodes = -2146368485i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNoAccessToUNC: MTSAdminErrorCodes = -2146368484i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrDllLoadFailed: MTSAdminErrorCodes = -2146368483i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadRegistryLibID: MTSAdminErrorCodes = -2146368482i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrPackDirNotFound: MTSAdminErrorCodes = -2146368481i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrTreatAs: MTSAdminErrorCodes = -2146368480i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadForward: MTSAdminErrorCodes = -2146368479i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrBadIID: MTSAdminErrorCodes = -2146368478i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrRegistrarFailed: MTSAdminErrorCodes = -2146368477i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileDoesNotExist: MTSAdminErrorCodes = -2146368476i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileLoadDLLFail: MTSAdminErrorCodes = -2146368475i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileGetClassObj: MTSAdminErrorCodes = -2146368474i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileClassNotAvail: MTSAdminErrorCodes = -2146368473i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileBadTLB: MTSAdminErrorCodes = -2146368472i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileNotInstallable: MTSAdminErrorCodes = -2146368471i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNotChangeable: MTSAdminErrorCodes = -2146368470i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrNotDeletable: MTSAdminErrorCodes = -2146368469i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrSession: MTSAdminErrorCodes = -2146368468i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsErrCompFileNoRegistrar: MTSAdminErrorCodes = -2146368460i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub type MTSPackageExportOptions = i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsExportUsers: MTSPackageExportOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub type MTSPackageInstallOptions = i32;
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`*"]
pub const mtsInstallUsers: MTSPackageInstallOptions = 1i32;