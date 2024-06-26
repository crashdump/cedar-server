use cedar_policy_core::parser::err::ParseErrors;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Policy {
    pub id: String,
    pub content: String,
}

impl From<cedar_policy::Policy> for Policy {
    fn from(policy: cedar_policy::Policy) -> Self {
        Policy {
            id: policy.id().to_string(),
            content: policy.to_string(),
        }
    }
}

impl TryInto<cedar_policy::Policy> for &Policy {
    type Error = ParseErrors;

    fn try_into(self) -> Result<cedar_policy::Policy, Self::Error> {
        cedar_policy::Policy::parse(Some(self.id.clone()), self.content.clone())
    }
}
