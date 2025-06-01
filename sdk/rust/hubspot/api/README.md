# NOTE
## Simple Cases

- compoenents.responses.*.description = ""


## Complicated Cases

Multiple Types

```json
"type": "object",
"properties": {
  "type" : "object",
  "additionalProperties" : {
    "type" : "string",
    "nullable" : true
  }
}
```

Multiple Response Types

```json
"responses": {
  "200": {
    "description": "successful operation",
    "content": {
      "application/json": {
        "schema": {
          "$ref": "#/components/schemas/BatchResponseSimplePublicObject"
        }
      }
    }
  },
  "207": {
    "description": "multiple statuses",
    "content": {
      "application/json": {
        "schema": {
          "$ref": "#/components/schemas/BatchResponseSimplePublicObjectWithErrors"
        }
      }
    }
  },
  "default": {
    "$ref": "#/components/responses/Error"
  }
}
```