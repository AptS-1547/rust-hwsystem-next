# API Documentation

## 1. Authentication

### Login
- **Endpoint:** `POST /api/v1/auth/login`
- **Description:** 用户登录
- **Request Body:**
```json
{
  "username": "student123",
  "password": "hashed_password123"
}
```
- **Response:**
```json
{
  "code": 0,
  "msg": "登录成功",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIs...",
    "user": {
      "id": 1001,
      "username": "student123",
      "role": "student"
    }
  }
}
```

## 2. Homework Management

### Create Homework
- **Endpoint:** `POST /api/v1/homeworks`
- **Description:** 创建新作业
- **Request Body:**
```json
{
  "title": "数学作业",
  "content": "完成习题1-10",
  "deadline": "2023-12-31 23:59"
}
```

### List Homework
- **Endpoint:** `GET /api/v1/homeworks`
- **Description:** 列出所有作业
- **Query Params:** `page=1&size=10`

### View Homework
- **Endpoint:** `GET /api/v1/homeworks/{homework_id}`
- **Description:** 查看作业详情

### Modify Homework
- **Endpoint:** `PUT /api/v1/homeworks/{homework_id}`
- **Description:** 修改作业
- **Request Body:**
```json
{
  "title": "更新后的标题",
  "content": "更新后的内容"
}
```

### Delete Homework
- **Endpoint:** `DELETE /api/v1/homeworks/{homework_id}`
- **Description:** 删除作业

### Submit Homework
- **Endpoint:** `POST /api/v1/homeworks/{homework_id}/submissions`
- **Description:** 提交作业
- **Request Body:**
```json
{
  "content": "作业内容...",
  "attachments": ["file1.pdf", "file2.jpg"]
}
```

### Reply to Homework
- **Endpoint:** `POST /api/v1/submissions/{submission_id}/feedback`
- **Description:** 批改作业
- **Request Body:**
```json
{
  "score": 95,
  "feedback": "完成得很好！"
}
```

## 3. Submission Management

### List Submissions by Time
- **Endpoint:** `GET /api/v1/homeworks/{homework_id}/submissions`
- **Description:** 按时间排序提交
- **Query Params:** `order=desc`

### List All Submissions
- **Endpoint:** `GET /api/v1/homeworks/{homework_id}/submissions`
- **Description:** 按学生姓名排序提交
- **Query Params:** `order=asc`

### Export Submissions
- **Endpoint:** `POST /api/v1/homeworks/{homework_id}/submissions/export`
- **Description:** 导出提交统计
- **Request Body:**
```json
{
  "students": [1001, 1002, 1003, ...]
}
```

## 4. System Management

### View Logs
- **Endpoint:** `GET /api/v1/logs`
- **Description:** 查看系统日志
- **Query Params:** `type=error&date=2023-10-01`

### Modify System Settings
- **Endpoint:** `PUT /api/v1/system/settings`
- **Description:** 修改系统设置
- **Request Body:**
```json
{
  "max_file_size": 10,
  "allowed_types": [".pdf", ".docx"],
  "max_attachments": 3
}
```

## 5. User Management

### List Users
- **Endpoint:** `GET /api/v1/users`
- **Description:** 列出所有用户
- **Query Params:** `role=student`

### Add User
- **Endpoint:** `POST /api/v1/users`
- **Description:** 添加用户
- **Request Body:**
```json
{
  "username": "new_user",
  "password": "init_password",
  "role": "teacher",
  "email": "user@example.com"
}
```

### Delete User
- **Endpoint:** `DELETE /api/v1/users/{user_id}`
- **Description:** 删除用户

### Modify User
- **Endpoint:** `PUT /api/v1/users/{user_id}`
- **Description:** 修改用户信息
- **Request Body:**
```json
{
  "role": "admin",
  "status": "active"
}
```
