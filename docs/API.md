# Turnkeeper API Documentation

This document provides an overview of the central API endpoints for the Turnkeeper backend. It describes the available routes, request/response formats, and authentication requirements.

## Error Handling

- All errors are returned as JSON with an appropriate HTTP status code and error message.

**Example error response:**

```json
{
    "error": "Invalid credentials"
}
```

###

## Authentication

Most endpoints require authentication via a JSON Web Token (JWT). Obtain a token using the `/user/login` or `/user/register` endpoints and include it in the `Authorization` header as `Bearer <token>`.

### POST `/user/login`

Authenticate a user and receive a JWT.

**Request Body:**

```json
{
    "name": "string (required)",
    "password": "string (required)"
}
```

**Response:**

```json
{
    "token": "string"
}
```

- Returns 401 if credentials are invalid.

---

### POST `/user/register`

Register a new user and receive a JWT.

**Request Body:**

```json
{
    "name": "string (required)",
    "password": "string (required)"
}
```

**Response:**

```json
{
    "token": "string"
}
```

- Returns 400 if the username is already taken or input is invalid.
