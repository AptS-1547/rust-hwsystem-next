pub enum ErrorCode {
    // 成功
    Success = 0, // 成功

    // 通用错误
    BadRequest = 1000,          // 错误的请求
    Unauthorized = 1001,        // 未授权访问
    NotFound = 1004,            // 未找到资源
    InternalServerError = 1005, // 内部服务器错误
    NotImplemented = 1006,      // 未实现的功能
    Conflict = 1009,            // 冲突 (资源已存在)

    // Auth 错误
    AuthFailed = 2000,     // 身份验证失败
    RegisterFailed = 2001, // 注册失败

    // 文件相关错误
    FileNotFound = 3000,              // 文件未找到
    FileUploadFailed = 3001,          // 文件上传失败
    FileTypeNotAllowed = 3002,        // 文件类型不被允许
    FileSizeExceeded = 3003,          // 文件大小超出限制
    MuitifileUploadNotAllowed = 3004, // 不允许多文件上传

    // 用户相关错误
    UserNotFound = 4000,            // 用户未找到
    UserAlreadyExists = 4001,       // 用户已存在
    UserUpdateFailed = 4002,        // 用户更新失败
    UserDeleteFailed = 4003,        // 用户删除失败
    UserCreationFailed = 4004,      // 用户创建失败
    CanNotDeleteCurrentUser = 4005, // 不能删除当前用户

    UserNameInvalid = 4010,        // 用户名无效
    UserNameAlreadyExists = 4011,  // 用户名已存在
    UserEmailInvalid = 4012,       // 用户邮箱无效
    UserEmailAlreadyExists = 4013, // 用户邮箱已存在、

    // 班级相关错误
    ClassNotFound = 5000,          // 班级未找到
    ClassAlreadyExists = 5001,     // 班级已存在
    ClassCreationFailed = 5002,    // 班级创建失败
    ClassUpdateFailed = 5003,      // 班级更新失败
    ClassDeleteFailed = 5004,      // 班级删除失败
    ClassPermissionDenied = 5005,  // 班级权限被拒绝
    ClassJoinFailed = 5010,        // 加入班级失败
    ClassInviteCodeInvalid = 5011, // 班级邀请码无效
    ClassAlreadyJoined = 5012,     // 已经加入该班级
    ClassJoinForbidden = 5013,     // 加入班级被禁止
}
