use serde::{Deserialize, Serialize};

/// Request parameters for creating a new user.
///
/// Contains the required and optional fields for user creation.
/// Email is the only required field.
#[derive(Debug, Serialize)]
pub struct CreateUserRequest<'a> {
    /// User's email address (required)
    pub email: &'a str,
    /// User's first name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    /// User's last name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    /// User's personal name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_name: Option<&'a str>,
    /// User's phone number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// User's custom parameters as JSON string (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_params: Option<&'a str>,
}

impl<'a> CreateUserRequest<'a> {
    /// Create a new user creation request with the required email field.
    pub fn new(email: &'a str) -> Self {
        Self {
            email,
            first_name: None,
            last_name: None,
            personal_name: None,
            phone: None,
            custom_params: None,
        }
    }

    /// Set the first name for the user.
    pub fn with_first_name(mut self, first_name: &'a str) -> Self {
        self.first_name = Some(first_name);
        self
    }

    /// Set the last name for the user.
    pub fn with_last_name(mut self, last_name: &'a str) -> Self {
        self.last_name = Some(last_name);
        self
    }

    /// Set the personal name for the user.
    pub fn with_personal_name(mut self, personal_name: &'a str) -> Self {
        self.personal_name = Some(personal_name);
        self
    }

    /// Set the phone number for the user.
    pub fn with_phone(mut self, phone: &'a str) -> Self {
        self.phone = Some(phone);
        self
    }

    /// Set custom parameters for the user (as JSON string).
    pub fn with_custom_params(mut self, custom_params: &'a str) -> Self {
        self.custom_params = Some(custom_params);
        self
    }
}

/// Request parameters for updating an existing user.
///
/// Contains the UID of the user to update and optional fields to modify.
#[derive(Debug, Serialize)]
pub struct UpdateUserRequest<'a> {
    /// User's unique identifier (required)
    pub uid: &'a str,
    /// User's email address (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// User's first name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    /// User's last name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    /// User's personal name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_name: Option<&'a str>,
    /// User's phone number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// User's custom parameters as JSON string (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_params: Option<&'a str>,
}

impl<'a> UpdateUserRequest<'a> {
    /// Create a new user update request with the required UID.
    pub fn new(uid: &'a str) -> Self {
        Self {
            uid,
            email: None,
            first_name: None,
            last_name: None,
            personal_name: None,
            phone: None,
            custom_params: None,
        }
    }

    /// Set the email for the user.
    pub fn with_email(mut self, email: &'a str) -> Self {
        self.email = Some(email);
        self
    }

    /// Set the first name for the user.
    pub fn with_first_name(mut self, first_name: &'a str) -> Self {
        self.first_name = Some(first_name);
        self
    }

    /// Set the last name for the user.
    pub fn with_last_name(mut self, last_name: &'a str) -> Self {
        self.last_name = Some(last_name);
        self
    }

    /// Set the personal name for the user.
    pub fn with_personal_name(mut self, personal_name: &'a str) -> Self {
        self.personal_name = Some(personal_name);
        self
    }

    /// Set the phone number for the user.
    pub fn with_phone(mut self, phone: &'a str) -> Self {
        self.phone = Some(phone);
        self
    }

    /// Set custom parameters for the user (as JSON string).
    pub fn with_custom_params(mut self, custom_params: &'a str) -> Self {
        self.custom_params = Some(custom_params);
        self
    }
}

/// Request parameters for listing users with filtering and pagination.
#[derive(Debug, Serialize, Default)]
pub struct ListUserRequest<'a> {
    /// Maximum number of results to return (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Offset for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Search query string (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<&'a str>,
    /// Filter by user status (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
    /// Order by field (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<&'a str>,
    /// Order direction (asc/desc) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_fields: Option<CustomFieldQuery>,
}

impl<'a> ListUserRequest<'a> {
    /// Create a new user list request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit for pagination.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset for pagination.
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set the search query string.
    pub fn with_query(mut self, q: &'a str) -> Self {
        self.q = Some(q);
        self
    }

    /// Set the status filter.
    pub fn with_status(mut self, status: &'a str) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the order by field.
    pub fn with_order_by(mut self, order_by: &'a str) -> Self {
        self.order_by = Some(order_by);
        self
    }

    /// Set the order direction.
    pub fn with_order_direction(mut self, order_direction: &'a str) -> Self {
        self.order_direction = Some(order_direction);
        self
    }
}

/// Request parameters for searching users.
#[derive(Debug, Serialize, Default)]
pub struct SearchUserRequest<'a> {
    /// Email to search for (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,
    /// UID to search for (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<&'a str>,
    /// Maximum number of results to return (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    /// Offset for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_fields: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum Source {
    VX,
    CF,
}

/// See https://docs.piano.io/faq-article/how-to-search-for-users-who-updated-their-custom-fields-during-a-daterange-via-api/
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ResponseTimeQuery {
    #[serde(rename = "BETWEEN")]
    Between {
        /// %Y-%m-%d
        more: String,
        /// %Y-%m-%d
        less: String,
    },
    #[serde(rename = "MORE")]
    More {
        /// %Y-%m-%d
        more: String,
    },
    #[serde(rename = "MORE")]
    Less {
        /// %Y-%m-%d
        less: String,
    },
    #[serde(rename = "CURRENT")]
    Current,
    #[serde(rename = "EQUAL")]
    Equal {
        /// %Y-%m-%d
        equal: String,
    },
}

impl ResponseTimeQuery {
    pub fn between(more: &str, less: &str) -> Self {
        Self::Between {
            more: more.to_string(),
            less: less.to_string(),
        }
    }
    pub fn more(more: &str) -> Self {
        Self::More {
            more: more.to_string(),
        }
    }
    pub fn less(less: &str) -> Self {
        Self::Less {
            less: less.to_string(),
        }
    }
    pub fn equal(equal: &str) -> Self {
        Self::Equal {
            equal: equal.to_string(),
        }
    }
    pub fn current() -> Self {
        Self::Current
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "data_type")]
pub enum CustomFieldQuery {
    #[serde(rename = "SINGLE_SELECT_LIST")]
    SingleSelectList {
        field_name: String,
        condition: SingleSelectListCondition,
        #[serde(skip_serializing_if = "Option::is_none")]
        response_time: Option<ResponseTimeQuery>,
        #[serde(skip_serializing_if = "Option::is_none")]
        field_title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
    },
    #[serde(rename = "TEXT")]
    Text {
        field_name: String,
        condition: TextCondition,
        #[serde(skip_serializing_if = "Option::is_none")]
        response_time: Option<ResponseTimeQuery>,
        #[serde(skip_serializing_if = "Option::is_none")]
        field_title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
    },
    #[serde(rename = "BOOLEAN")]
    Boolean {
        field_name: String,
        condition: BooleanCondition,
        #[serde(skip_serializing_if = "Option::is_none")]
        response_time: Option<ResponseTimeQuery>,
        #[serde(skip_serializing_if = "Option::is_none")]
        field_title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
    },
    #[serde(rename = "NUMBER")]
    Number {
        field_name: String,
        condition: NumberCondition,
        #[serde(skip_serializing_if = "Option::is_none")]
        response_time: Option<ResponseTimeQuery>,
        #[serde(skip_serializing_if = "Option::is_none")]
        field_title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
    },
    #[serde(rename = "ISO_DATE")]
    ISODate {
        field_name: String,
        condition: DateCondition,
        #[serde(skip_serializing_if = "Option::is_none")]
        response_time: Option<ResponseTimeQuery>,
        #[serde(skip_serializing_if = "Option::is_none")]
        field_title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
    },
}

impl CustomFieldQuery {
    pub fn data_type(&self) -> CustomFieldDataType {
        match self {
            Self::SingleSelectList { .. } => CustomFieldDataType::SingleSelectList,
            Self::Text { .. } => CustomFieldDataType::Text,
            Self::Boolean { .. } => CustomFieldDataType::Boolean,
            Self::Number { .. } => CustomFieldDataType::Number,
            Self::ISODate { .. } => CustomFieldDataType::ISODate,
        }
    }
    pub fn field_name(&self) -> &str {
        match self {
            Self::SingleSelectList { field_name, .. } => field_name,
            Self::Text { field_name, .. } => field_name,
            Self::Boolean { field_name, .. } => field_name,
            Self::Number { field_name, .. } => field_name,
            Self::ISODate { field_name, .. } => field_name,
        }
    }
    pub fn response_time(&self) -> Option<&ResponseTimeQuery> {
        match self {
            Self::SingleSelectList { response_time, .. } => response_time.as_ref(),
            Self::Text { response_time, .. } => response_time.as_ref(),
            Self::Boolean { response_time, .. } => response_time.as_ref(),
            Self::Number { response_time, .. } => response_time.as_ref(),
            Self::ISODate { response_time, .. } => response_time.as_ref(),
        }
    }
    pub fn enabled(&self) -> Option<&bool> {
        match self {
            Self::SingleSelectList { enabled, .. } => enabled.as_ref(),
            Self::Text { enabled, .. } => enabled.as_ref(),
            Self::Boolean { enabled, .. } => enabled.as_ref(),
            Self::Number { enabled, .. } => enabled.as_ref(),
            Self::ISODate { enabled, .. } => enabled.as_ref(),
        }
    }
    pub fn with_field_title(self, field_title: &str) -> Self {
        match self {
            Self::SingleSelectList {
                condition,
                field_name,
                response_time,
                enabled,
                ..
            } => Self::SingleSelectList {
                field_title: Some(field_title.to_string()),
                condition,
                field_name,
                response_time,
                enabled,
            },
            Self::Text {
                condition,
                field_name,
                response_time,
                enabled,
                ..
            } => Self::Text {
                field_title: Some(field_title.to_string()),
                condition,
                field_name,
                response_time,
                enabled,
            },
            Self::Boolean {
                condition,
                field_name,
                response_time,
                enabled,
                ..
            } => Self::Boolean {
                field_title: Some(field_title.to_string()),
                condition,
                field_name,
                response_time,
                enabled,
            },
            Self::Number {
                condition,
                field_name,
                response_time,
                enabled,
                ..
            } => Self::Number {
                field_title: Some(field_title.to_string()),
                condition,
                field_name,
                response_time,
                enabled,
            },
            Self::ISODate {
                condition,
                field_name,
                response_time,
                enabled,
                ..
            } => Self::ISODate {
                field_title: Some(field_title.to_string()),
                condition,
                field_name,
                response_time,
                enabled,
            },
        }
    }
    pub fn with_response_time(self, response_time: ResponseTimeQuery) -> Self {
        match self {
            Self::SingleSelectList {
                field_name,
                condition,
                field_title,
                enabled,
                ..
            } => Self::SingleSelectList {
                response_time: Some(response_time),
                field_name,
                condition,
                field_title,
                enabled,
            },
            Self::Text {
                field_name,
                condition,
                field_title,
                enabled,
                ..
            } => Self::Text {
                response_time: Some(response_time),
                field_name,
                condition,
                field_title,
                enabled,
            },
            Self::Boolean {
                field_name,
                condition,
                field_title,
                enabled,
                ..
            } => Self::Boolean {
                response_time: Some(response_time),
                field_name,
                condition,
                field_title,
                enabled,
            },
            Self::Number {
                field_name,
                condition,
                field_title,
                enabled,
                ..
            } => Self::Number {
                response_time: Some(response_time),
                field_name,
                condition,
                field_title,
                enabled,
            },
            Self::ISODate {
                field_name,
                condition,
                field_title,
                enabled,
                ..
            } => Self::ISODate {
                response_time: Some(response_time),
                field_name,
                condition,
                field_title,
                enabled,
            },
        }
    }
    pub fn number_eq(field_name: &str, value: &str) -> Self {
        Self::Number {
            field_name: field_name.to_string(),
            condition: NumberCondition::equal(value),
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn number_between(field_name: &str, more: &str, less: &str) -> Self {
        Self::Number {
            field_name: field_name.to_string(),
            condition: NumberCondition::between(more, less),
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn number_more(field_name: &str, more: &str) -> Self {
        Self::Number {
            field_name: field_name.to_string(),
            condition: NumberCondition::more(more),
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn number_less(field_name: &str, less: &str) -> Self {
        Self::Number {
            field_name: field_name.to_string(),
            condition: NumberCondition::less(less),
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn single_select_list_contains_exact(field_name: &str, value: &str) -> Self {
        Self::SingleSelectList {
            field_name: field_name.to_string(),
            condition: SingleSelectListCondition::options(ContainExactOne::string(value)),
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn text_like(field_name: &str, like: &str) -> Self {
        Self::Text {
            field_name: field_name.to_string(),
            condition: TextCondition::Like {
                like: like.to_string(),
            },
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn text_eq(field_name: &str, value: &str) -> Self {
        Self::Text {
            field_name: field_name.to_string(),
            condition: TextCondition::Equal {
                equal: value.to_string(),
            },
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn text_empty(field_name: &str) -> Self {
        Self::Text {
            field_name: field_name.to_string(),
            condition: TextCondition::Empty,
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn text_any(field_name: &str) -> Self {
        Self::Text {
            field_name: field_name.to_string(),
            condition: TextCondition::Any,
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn bool_eq(field_name: &str, value: bool) -> Self {
        Self::Boolean {
            field_name: field_name.to_string(),
            condition: BooleanCondition::Equal { equal: value },
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn bool_empty(field_name: &str) -> Self {
        Self::Boolean {
            field_name: field_name.to_string(),
            condition: BooleanCondition::empty(),
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn bool_any(field_name: &str) -> Self {
        Self::Boolean {
            field_name: field_name.to_string(),
            condition: BooleanCondition::any(),
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn date_between(field_name: &str, more: &str, less: &str) -> Self {
        Self::ISODate {
            field_name: field_name.to_string(),
            condition: DateCondition::between(more, less),
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn date_equal(field_name: &str, equal: &str) -> Self {
        Self::ISODate {
            field_name: field_name.to_string(),
            condition: DateCondition::equal(equal),
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum BooleanCondition {
    #[serde(rename = "EMPTY")]
    Empty,
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "EQUAL")]
    Equal { equal: bool },
}
impl BooleanCondition {
    pub fn empty() -> Self {
        Self::Empty
    }
    pub fn any() -> Self {
        Self::Any
    }
    pub fn equal(equal: bool) -> Self {
        Self::Equal { equal }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum DateCondition {
    #[serde(rename = "EMPTY")]
    Empty,
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "EQUAL")]
    Equal {
        /// format: %Y-%m-%d
        equal: String,
    },
    #[serde(rename = "BETWEEN")]
    Between {
        /// format: %Y-%m-%d
        more: String,
        /// format: %Y-%m-%d
        less: String,
    },
}
impl DateCondition {
    pub fn empty() -> Self {
        Self::Empty
    }
    pub fn any() -> Self {
        Self::Any
    }
    pub fn equal(equal: &str) -> Self {
        Self::Equal {
            equal: equal.to_string(),
        }
    }
    pub fn between(more: &str, less: &str) -> Self {
        Self::Between {
            more: more.to_string(),
            less: less.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum SingleSelectListCondition {
    #[serde(rename = "EQUAL")]
    Equal {
        #[serde(rename = "optionsEqual")]
        options_equal: ContainExactOne,
    },
    #[serde(rename = "EMPTY")]
    Empty,
    #[serde(rename = "ANY")]
    Any,
}

impl SingleSelectListCondition {
    pub fn as_field_query(self, field_name: &str) -> CustomFieldQuery {
        CustomFieldQuery::SingleSelectList {
            field_name: field_name.to_string(),
            condition: self,
            response_time: None,
            field_title: None,
            enabled: None,
        }
    }
    pub fn options(options_equal: ContainExactOne) -> Self {
        Self::Equal { options_equal }
    }
    pub fn empty() -> Self {
        Self::Empty
    }
    pub fn any() -> Self {
        Self::Any
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ContainExactOne {
    String {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
        value: String,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum TextCondition {
    #[serde(rename = "EMPTY")]
    Empty,
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "LIKE")]
    Like { like: String },
    #[serde(rename = "EQUAL")]
    Equal { equal: String },
    #[serde(rename = "EXACT_MATCH")]
    ExactMatch { exact_match: String },
}

#[derive(Debug, Serialize)]
pub enum NumberCondition {
    #[serde(rename = "EMPTY")]
    Empty,
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "EQUAL")]
    Equal { equal: String },
    #[serde(rename = "MORE")]
    More { more: String },
    #[serde(rename = "LESS")]
    Less { less: String },
    #[serde(rename = "BETWEEN")]
    Between { more: String, less: String },
}
impl NumberCondition {
    pub fn empty() -> Self {
        Self::Empty
    }
    pub fn any() -> Self {
        Self::Any
    }
    pub fn equal(equal: &str) -> Self {
        Self::Equal {
            equal: equal.to_string(),
        }
    }
    pub fn more(more: &str) -> Self {
        Self::More {
            more: more.to_string(),
        }
    }
    pub fn less(less: &str) -> Self {
        Self::Less {
            less: less.to_string(),
        }
    }
    pub fn between(more: &str, less: &str) -> Self {
        Self::Between {
            more: more.to_string(),
            less: less.to_string(),
        }
    }
}

impl ContainExactOne {
    pub fn string(value: &str) -> Self {
        Self::String {
            id: None,
            value: value.to_string(),
        }
    }
    pub fn string_with_id(id: u32, value: &str) -> Self {
        Self::String {
            id: Some(id),
            value: value.to_string(),
        }
    }
}

impl<'a> SearchUserRequest<'a> {
    /// Create a new user search request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Search by email.
    pub fn with_email(mut self, email: &'a str) -> Self {
        self.email = Some(email);
        self
    }

    /// Search by source.
    pub fn with_source(mut self, source: Source) -> Self {
        self.source = Some(source);
        self
    }
    pub fn with_custom_field_queries(mut self, custom_field_queries: &[CustomFieldQuery]) -> Self {
        let value = serde_json::to_string(custom_field_queries).expect("OK");
        self.custom_fields = Some(value);
        self
    }

    /// Search by UID.
    pub fn with_uid(mut self, uid: &'a str) -> Self {
        self.uid = Some(uid);
        self
    }

    /// Set the limit for pagination.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset for pagination.
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Response wrapper for single user operations.
#[derive(Debug, Deserialize, Clone)]
pub(super) struct UserResult {
    /// The user data - Piano API can return with different field names
    #[serde(alias = "User", alias = "user")]
    pub user: User,
}

/// Response for user list operations.
#[derive(Debug, Deserialize, Clone)]
pub struct ListUserResult {
    /// Array of users
    #[serde(alias = "Users", alias = "users")]
    pub users: Vec<User>,
}

/// Represents a Piano platform user.
///
/// Contains user information including personal details and platform-specific data.
#[derive(Debug, Deserialize, Clone)]
pub struct User {
    /// User's unique identifier
    uid: String,
    /// User's email address
    email: String,
    /// User's first name (optional)
    #[serde(default)]
    first_name: Option<String>,
    /// User's last name (optional)
    #[serde(default)]
    last_name: Option<String>,
    /// User's personal name (optional)
    #[serde(default)]
    personal_name: Option<String>,
    /// User's display name (optional)
    #[serde(default)]
    display_name: Option<String>,
    /// User's phone number (optional)
    #[serde(default)]
    phone: Option<String>,
    /// Timestamp when user was created (UNIX timestamp)
    create_date: i64,
    /// Timestamp when user was last updated (UNIX timestamp)
    #[serde(default)]
    update_date: Option<i64>,
    /// User's current status
    #[serde(default)]
    status: Option<String>,
    /// Custom parameters as JSON string (optional)
    #[serde(default)]
    custom_fields: Vec<CustomField>,
    #[serde(default)]
    reset_password_email_sent: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CustomField {
    archived: bool,
    attribute: CustomFieldAttribute,
    #[serde(default)]
    comment: String,
    /// Example: "2024-11-06T02:54:24.000+00:00",
    created: Option<String>,
    #[serde(rename = "dataType")]
    data_type: CustomFieldDataType,
    #[serde(rename = "defaultSortOrder")]
    default_sort_order: Option<usize>,
    #[serde(rename = "sortOrder")]
    sort_order: Option<usize>,
    editable: bool,
    #[serde(default)]
    #[serde(rename = "favouriteOptions")]
    favourite_options: Option<Vec<String>>,
    #[serde(default)]
    options: Vec<String>,
    #[serde(rename = "emailCreator")]
    email_creator: Option<String>,
    #[serde(rename = "fieldName")]
    field_name: String,
    title: String,
    tooltip: Option<Tooltip>,
    value: Option<String>,
}

impl CustomField {
    /// Get whether the custom field is archived.
    pub fn archived(&self) -> bool {
        self.archived
    }

    pub fn as_vec(&self) -> Option<Vec<String>> {
        match self.data_type() {
            CustomFieldDataType::SingleSelectList => match &self.value {
                None => Some(vec![]),
                Some(str) => Some(
                    str.trim_start_matches("[")
                        .trim_end_matches("]")
                        .split(",")
                        .into_iter()
                        .map(Self::unquote_str)
                        .collect::<Vec<_>>(),
                ),
            },
            CustomFieldDataType::Boolean
            | CustomFieldDataType::MultiSelectList
            | CustomFieldDataType::Number
            | CustomFieldDataType::ISODate
            | CustomFieldDataType::Text => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self.data_type() {
            CustomFieldDataType::Boolean => self.value().unwrap_or_default().parse::<bool>().ok(),
            CustomFieldDataType::ISODate
            | CustomFieldDataType::Number
            | CustomFieldDataType::SingleSelectList
            | CustomFieldDataType::MultiSelectList
            | CustomFieldDataType::Text => None,
        }
    }
    fn unquote_str(s: &str) -> String {
        s.trim_start_matches("\"")
            .trim_end_matches("\"")
            .to_string()
    }

    /// Get the custom field attribute.
    pub fn attribute(&self) -> &CustomFieldAttribute {
        &self.attribute
    }

    /// Get the custom field comment.
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Get the custom field creation timestamp.
    pub fn created(&self) -> Option<&str> {
        self.created.as_deref()
    }

    /// Get the custom field data type.
    pub fn data_type(&self) -> &CustomFieldDataType {
        &self.data_type
    }

    /// Get the default sort order.
    pub fn default_sort_order(&self) -> Option<usize> {
        self.default_sort_order
    }

    /// Get the sort order.
    pub fn sort_order(&self) -> Option<usize> {
        self.sort_order
    }

    /// Get whether the custom field is editable.
    pub fn editable(&self) -> bool {
        self.editable
    }

    /// Get the favourite options.
    pub fn favourite_options(&self) -> Option<&[String]> {
        self.favourite_options.as_deref()
    }

    /// Get the options.
    pub fn options(&self) -> &[String] {
        &self.options
    }

    /// Get the email creator.
    pub fn email_creator(&self) -> Option<&str> {
        self.email_creator.as_deref()
    }

    /// Get the field name.
    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    /// Get the title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the tooltip.
    pub fn tooltip(&self) -> Option<&Tooltip> {
        self.tooltip.as_ref()
    }

    /// Get the value.
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CustomFieldAttribute {
    #[serde(default)]
    multiline: Option<bool>,
}

impl CustomFieldAttribute {
    /// Get whether multiline is enabled.
    pub fn multiline(&self) -> Option<bool> {
        self.multiline
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tooltip {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    text: String,
    #[serde(rename = "type")]
    tpe: String,
}

impl Tooltip {
    /// Get whether the tooltip is enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Get the tooltip text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the tooltip type.
    pub fn tpe(&self) -> &str {
        &self.tpe
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum CustomFieldDataType {
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "ISO_DATE")]
    ISODate,
    #[serde(rename = "SINGLE_SELECT_LIST")]
    SingleSelectList,
    #[serde(rename = "MULTI_SELECT_LIST")]
    MultiSelectList,
    #[serde(rename = "NUMBER")]
    Number,
}

impl User {
    /// Get the user's unique identifier.
    pub fn uid(&self) -> &str {
        &self.uid
    }

    /// Get the user's email address.
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Get the user's first name.
    pub fn first_name(&self) -> Option<&str> {
        self.first_name.as_deref()
    }

    /// Get the user's last name.
    pub fn last_name(&self) -> Option<&str> {
        self.last_name.as_deref()
    }

    /// Get the user's personal name.
    pub fn personal_name(&self) -> Option<&str> {
        self.personal_name.as_deref()
    }

    /// Get the user's display name.
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    /// Get the user's phone number.
    pub fn phone(&self) -> Option<&str> {
        self.phone.as_deref()
    }

    /// Get the user's creation timestamp.
    pub fn create_date(&self) -> i64 {
        self.create_date
    }

    /// Get the user's last update timestamp.
    pub fn update_date(&self) -> Option<i64> {
        self.update_date
    }

    /// Get the user's status.
    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    /// Get the user's custom fields.
    pub fn custom_fields(&self) -> &[CustomField] {
        &self.custom_fields
    }

    /// Get whether a reset password email was sent.
    pub fn reset_password_email_sent(&self) -> bool {
        self.reset_password_email_sent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PianoPaginated, PianoResponse};
    #[test]
    fn sanity_check_custom_field_decoding() {
        let value = serde_json::json!([
            {
                "archived": false,
                "attribute": {
                  "multiline": false
                },
                "comment": "**MASKED**",
                "created": null,
                "dataType": "TEXT",
                "defaultSortOrder": null,
                "editable": false,
                "emailCreator": null,
                "favouriteOptions": [],
                "fieldDefinitionId": null,
                "fieldName": "**MASKED**",
                "identityId": null,
                "options": [],
                "sortOrder": null,
                "title": "**MASKED**",
                "tooltip": {
                  "enabled": false,
                  "text": "",
                  "type": "InfoIcon"
                },
                "validators": "[]",
                "value": null
            },
            {
                "archived": false,
                "attribute": {},
                "comment": "",
                "created": "2024-11-06T02:55:24.000+00:00",
                "dataType": "BOOLEAN",
                "defaultSortOrder": null,
                "editable": true,
                "emailCreator": "test@example.com",
                "favouriteOptions": null,
                "fieldDefinitionId": null,
                "fieldName": "**MASKED**",
                "identityId": null,
                "options": [],
                "sortOrder": null,
                "title": "**MASKED**",
                "tooltip": {
                    "enabled": false,
                    "text": "",
                    "type": "InfoIcon"
                },
                "validators": "[]",
                "value": "true"
            },
            {
                "archived": false,
                "attribute": {},
                "comment": "",
                "created": "2024-12-23T01:12:24.000+00:00",
                "dataType": "SINGLE_SELECT_LIST",
                "defaultSortOrder": null,
                "editable": false,
                "emailCreator": "test@example.com",
                "favouriteOptions": null,
                "fieldDefinitionId": null,
                "fieldName": "**MASKED**",
                "identityId": null,
                "options": [
                    "**MASKED-A**",
                    "**MASKED-B**",
                    "**MASKED-C**",
                ],
                "sortOrder": null,
                "title": "**MASKED**",
                "tooltip": {
                    "enabled": false,
                    "text": "",
                    "type": "InfoIcon"
                },
                "validators": "[]",
                "value": "[\"**MASKED-A**\"]"
            }
        ]);
        let result = serde_json::from_value::<Vec<CustomField>>(value);
        assert!(result.is_ok())
    }
    #[test]
    fn sanity_check_single_select_list_custom_field_query_encoding() {
        let value = serde_json::json!(
            {
                "field_name": "occupation_status",
                "data_type": "SINGLE_SELECT_LIST",
                "condition": {
                  "type": "EQUAL",
                  "optionsEqual": {
                    "value": "Full-time work"
                  }
                }
              }
        );
        let one = serde_json::to_value(&CustomFieldQuery::single_select_list_contains_exact(
            "occupation_status",
            "Full-time work",
        ))
        .expect("OK");
        assert_eq!(one, value)
    }
    #[test]
    fn sanity_check_text_custom_field_query_encoding() {
        let value = serde_json::json!(
            {
                "field_name": "Text",
                "data_type": "TEXT",
                "condition": {
                    "type": "LIKE", // or "EQUAL" or "EXACT_MATCH"
                    "like": "Test"
                }
            }
        );
        let one = serde_json::to_value(&CustomFieldQuery::text_like("Text", "Test")).expect("OK");
        assert_eq!(one, value)
    }
    #[test]
    fn sanity_check_date_custom_field_query_encoding() {
        let value = serde_json::json!(
                {
                    "field_name": "age",
                    "data_type": "ISO_DATE",
                    "condition": {
                        "type": "BETWEEN",
                        "more": "2025-03-03",
                        "less": "2025-03-12"
                    }
                }
        );
        let result = serde_json::to_value(&CustomFieldQuery::date_between(
            "age",
            "2025-03-03",
            "2025-03-12",
        ))
        .expect("OK");
        assert_eq!(result, value)
    }
    #[test]
    fn sanity_check_custom_field_query_serialization() {
        let data = CustomFieldQuery::single_select_list_contains_exact("foo", "foo");
        let str = serde_json::to_string(&data).expect("OK");
        assert_eq!(
            str,
           "{\"data_type\":\"SINGLE_SELECT_LIST\",\"field_name\":\"foo\",\"condition\":{\"type\":\"EQUAL\",\"optionsEqual\":{\"value\":\"foo\"}}}"
        )
    }
    #[test]
    fn test_create_user_request_builder() {
        let request = CreateUserRequest::new("user@example.com")
            .with_first_name("John")
            .with_last_name("Doe")
            .with_personal_name("Johnny");

        assert_eq!(request.email, "user@example.com");
        assert_eq!(request.first_name, Some("John"));
        assert_eq!(request.last_name, Some("Doe"));
        assert_eq!(request.personal_name, Some("Johnny"));
    }

    #[test]
    fn test_list_user_request_builder() {
        let request = ListUserRequest::new()
            .with_limit(50)
            .with_offset(10)
            .with_query("search term");

        assert_eq!(request.limit, Some(50));
        assert_eq!(request.offset, Some(10));
        assert_eq!(request.q, Some("search term"));
    }

    #[test]
    fn test_user_deserialization() {
        let json = serde_json::json!({
            "uid": "12345",
            "email": "user@example.com",
            "first_name": "John",
            "last_name": "Doe",
            "personal_name": "Johnny",
            "create_date": 1640995200,
            "update_date": 1641081600
        });

        let user: User = serde_json::from_value(json).expect("Failed to deserialize user");
        assert_eq!(user.uid(), "12345");
        assert_eq!(user.email(), "user@example.com");
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert_eq!(user.personal_name(), Some("Johnny"));
    }

    #[test]
    fn sanity_check_list_users_codec() {
        let snapshot = include_str!("./list.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<PianoPaginated<ListUserResult>>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize user list: {:?}",
            value.err()
        );
        let response = value.unwrap();

        match response {
            PianoResponse::Succeed(paginated) => {
                assert_eq!(paginated.limit, 1);
                assert_eq!(paginated.offset, 0);
                assert!(paginated.total >= 165);
                assert!(paginated.count >= 1);

                if !paginated.value.users.is_empty() {
                    let user = &paginated.value.users[0];
                    assert_eq!(user.email(), "test@example.com");
                    assert_eq!(user.uid(), "***MASKED***");
                }
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }

    #[test]
    fn sanity_check_get_user_codec() {
        let snapshot = include_str!("./get.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<UserResult>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize user get: {:?}",
            value.err()
        );
        let response = value.unwrap();

        match response {
            PianoResponse::Succeed(data) => {
                assert_eq!(data.user.email(), "test@example.com");
                assert_eq!(data.user.uid(), "***MASKED***");
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }
}
