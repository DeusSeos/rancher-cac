use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use rancher_client::apis::{configuration::Configuration, Error, ResponseContent};
use reqwest::StatusCode;

use rancher_client::{
    apis::management_cattle_io_v3_api::{
        list_management_cattle_io_v3_role_template, ListManagementCattleIoV3RoleTemplateError,
    },
    models::io_cattle_managementv3_role_template::Context,
    models::{
        IoCattleManagementv3GlobalRoleRulesInner, IoCattleManagementv3RoleTemplate,
        IoCattleManagementv3RoleTemplateList, IoK8sApimachineryPkgApisMetaV1ObjectMeta,
    },
};

pub const RT_EXCLUDE_PATHS: &[&str] = &[
    "metadata.creationTimestamp",
    "metadata.finalizers",
    "metadata.generateName",
    "metadata.generation",
    "metadata.managedFields",
    "metadata.resourceVersion",
    "metadata.selfLink",
    "metadata.uid",
];



/// Get all role templates from an endpoint using the provided configuration
///
/// # Arguments
///
/// * `configuration` - The configuration to use for the request
///
/// # Returns
///
/// * `IoCattleManagementv3RoleTemplateList` - The list of role templates
///
/// # Errors
///
/// * `Error<ListManagementCattleIoV3RoleTemplateError>` - The error that occurred while trying to get the role templates
///
#[async_backtrace::framed]
pub async fn get_role_templates(
    configuration: &Configuration,
    field_selector: Option<&str>,
    label_selector: Option<&str>,
    limit: Option<i32>,
    resource_version: Option<&str>,
    resource_version_match: Option<&str>,
    continue_: Option<&str>,
) -> Result<IoCattleManagementv3RoleTemplateList, Error<ListManagementCattleIoV3RoleTemplateError>>
{
    let result = list_management_cattle_io_v3_role_template(
        configuration,
        None,
        None,
        continue_,
        field_selector,
        label_selector,
        limit,
        resource_version,
        resource_version_match,
        None,
        None,
        None,
    )
    .await;

    match result {
        Err(e) => {
            // TODO: Handle specific error cases
            Err(e)
        },
        Ok(response_content) => {
            // Match on the status code and deserialize accordingly
            match response_content.status {
                StatusCode::OK => {
                    // Try to deserialize the content into IoCattleManagementv3RoleTemplateList (Status200 case)
                    match serde_json::from_str(&response_content.content) {
                        Ok(data) => Ok(data),
                        Err(deserialize_err) => Err(Error::Serde(deserialize_err)),
                    }
                }
                _ => {
                    // If not status 200, treat as UnknownValue
                    match serde_json::from_str::<serde_json::Value>(&response_content.content) {
                        Ok(unknown_data) => {
                            // Handle the unknown response
                            Err(Error::ResponseError(ResponseContent {
                                status: response_content.status,
                                content: response_content.content,
                                entity: Some(
                                    ListManagementCattleIoV3RoleTemplateError::UnknownValue(
                                        unknown_data,
                                    ),
                                ),
                            }))
                        }
                        Err(deserialize_err) => Err(Error::Serde(deserialize_err)),
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RoleTemplate {

    /// Administrative if true, this RoleTemplate is used to grant administrative privileges. Default to false.
    /// This field is not set in the API, but is used to determine if the role template is administrative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builtin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_creator_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_creator_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_template_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<IoCattleManagementv3GlobalRoleRulesInner>>,
}

impl RoleTemplate {
    pub fn new(
        administrative: Option<bool>,
        annotations: Option<HashMap<String, String>>,
        builtin: Option<bool>,
        cluster_creator_default: Option<bool>,
        context: Option<Context>,
        description: Option<String>,
        display_name: Option<String>,
        external: Option<bool>,
        hidden: Option<bool>,
        labels: Option<HashMap<String, String>>,
        locked: Option<bool>,
        id: String,
        project_creator_default: Option<bool>,
        role_template_names: Option<Vec<String>>,
        rules: Option<Vec<IoCattleManagementv3GlobalRoleRulesInner>>,
    ) -> Self {
        RoleTemplate {
            annotations,
            administrative,
            builtin,
            cluster_creator_default,
            context,
            description,
            display_name,
            external,
            hidden,
            labels,
            locked,
            id,
            project_creator_default,
            role_template_names,
            rules,
        }
    }
}

impl TryFrom<IoCattleManagementv3RoleTemplate> for RoleTemplate {
    type Error = &'static str;

    fn try_from(value: IoCattleManagementv3RoleTemplate) -> Result<Self, Self::Error> {
        let metadata: IoK8sApimachineryPkgApisMetaV1ObjectMeta = value.metadata.ok_or("missing metadata")?;

        let administrative: Option<bool> = value.administrative;
        let annotations: Option<HashMap<String, String>> = metadata.annotations;
        let builtin: Option<bool> = value.builtin;
        let cluster_creator_default: Option<bool> = value.cluster_creator_default;
        let context = value.context;
        let description: Option<String> = value.description;
        let display_name: Option<String> = value.display_name;
        let external: Option<bool> = value.external;
        let hidden: Option<bool> = value.hidden;
        let labels: Option<HashMap<String, String>> = metadata.labels;
        let locked: Option<bool> = value.locked;
        let project_creator_default: Option<bool> = value.project_creator_default;
        let role_template_names: Option<Vec<String>> = value.role_template_names;
        let rules: Option<Vec<IoCattleManagementv3GlobalRoleRulesInner>> = value.rules;

        Ok(RoleTemplate {
            administrative,
            annotations,
            builtin,
            cluster_creator_default,
            context,
            description,
            display_name,
            external,
            hidden,
            id: metadata.name.ok_or("missing metadata.name")?,
            labels,
            locked,
            project_creator_default,
            role_template_names,
            rules,
        })
    }
}

impl TryFrom<RoleTemplate> for IoCattleManagementv3RoleTemplate {
    type Error = &'static str;

    fn try_from(value: RoleTemplate) -> Result<Self, Self::Error> {
        let metadata = IoK8sApimachineryPkgApisMetaV1ObjectMeta {
            annotations: value.annotations,
            labels: value.labels,
            name: Some(value.id.clone()),
            ..Default::default()
        };

        let context = value.context;
        let administrative: Option<bool> = value.administrative;
        let builtin: Option<bool> = value.builtin;
        let cluster_creator_default: Option<bool> = value.cluster_creator_default;
        let description: Option<String> = value.description;
        let display_name: Option<String> = value.display_name;
        let external: Option<bool> = value.external;
        let hidden: Option<bool> = value.hidden;
        let locked: Option<bool> = value.locked;
        let project_creator_default: Option<bool> = value.project_creator_default;
        let role_template_names: Option<Vec<String>> = value.role_template_names;
        let rules: Option<Vec<IoCattleManagementv3GlobalRoleRulesInner>> = value.rules;

        Ok(IoCattleManagementv3RoleTemplate {
            administrative,
            api_version: Some("management.cattle.io/v3".to_string()),
            builtin,
            cluster_creator_default,
            context,
            description,
            display_name,
            external,
            hidden,
            kind: Some("RoleTemplate".to_string()),
            locked,
            metadata: Some(metadata),
            project_creator_default,
            role_template_names,
            rules,
        })
    }
}

impl PartialEq<RoleTemplate> for IoCattleManagementv3RoleTemplate {
    fn eq(&self, other: &RoleTemplate) -> bool {
        let lhs = self.metadata.as_ref().and_then(|m| m.name.clone());
        let rhs = Some(other.id.clone());

        lhs == rhs
            && self.administrative == other.administrative
            && self.builtin == other.builtin
            && self.cluster_creator_default == other.cluster_creator_default
            && self.context == other.context
            && self.description == other.description
            && self.display_name == other.display_name
            && self.external == other.external
            && self.hidden == other.hidden
            && self.locked == other.locked
            && self.project_creator_default == other.project_creator_default
            && self.role_template_names == other.role_template_names
            && self.rules == other.rules
    }
}


impl PartialEq<IoCattleManagementv3RoleTemplate> for RoleTemplate {
    fn eq(&self, other: &IoCattleManagementv3RoleTemplate) -> bool {
        // let lhs = Some(self.id.clone());
        // let rhs = other.metadata.as_ref().and_then(|m| m.name.clone());

        // self.administrative == other.administrative
        //     && self.builtin == other.builtin
        //     && self.cluster_creator_default == other.cluster_creator_default
        //     && self.context == other.context
        //     && self.description == other.description
        //     && self.display_name == other.display_name
        //     && self.external == other.external
        //     && self.hidden == other.hidden
        //     && self.locked == other.locked
        //     && lhs == rhs
        //     && self.project_creator_default == other.project_creator_default
        //     && self.role_template_names == other.role_template_names
        //     && self.rules == other.rules


        other == self
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    fn sample_metadata(name: &str) -> IoK8sApimachineryPkgApisMetaV1ObjectMeta {
        IoK8sApimachineryPkgApisMetaV1ObjectMeta {
            name: Some(name.to_string()),
            ..Default::default()
        }
    }

    fn sample_role_template() -> RoleTemplate {
        RoleTemplate::new(
            Some(true), // administrative
            Some(std::collections::HashMap::new()), // annotations
            Some(false), // builtin
            Some(true), // cluster_creator_default
            Some(Context::Cluster), // context
            Some("A role template".to_string()), // description
            Some("Admin".to_string()), // display_name
            Some(false), // external
            Some(false), // hidden
            Some(std::collections::HashMap::new()), // labels
            Some(false), // locked
            "admin-template".to_string(), // id
            Some(false), // project_creator_default
            Some(vec!["base-template".to_string()]), // role_template_names
            Some(vec![]), // rules
        )
    }

    fn sample_iocattle_role_template() -> IoCattleManagementv3RoleTemplate {
        IoCattleManagementv3RoleTemplate {
            administrative: Some(true),
            api_version: Some("management.cattle.io/v3".to_string()),
            builtin: Some(false),
            cluster_creator_default: Some(true),
            context: Some(Context::Cluster),
            description: Some("A role template".to_string()),
            display_name: Some("Admin".to_string()),
            external: Some(false),
            hidden: Some(false),
            kind: Some("RoleTemplate".to_string()),
            locked: Some(false),
            metadata: Some(sample_metadata("admin-template")),
            project_creator_default: Some(false),
            role_template_names: Some(vec!["base-template".to_string()]),
            rules: Some(vec![]),
        }
    }

    #[test]
    fn test_iocattle_to_role_template_conversion_success() {
        let io_rt = sample_iocattle_role_template();
        let result = RoleTemplate::try_from(io_rt).unwrap();
        assert_eq!(result.id, "admin-template");
        assert_eq!(result.display_name.as_deref(), Some("Admin"));
        assert_eq!(result.project_creator_default, Some(false));
    }

    #[test]
    fn test_role_template_to_iocattle_conversion_success() {
        let rt = sample_role_template();
        let result = IoCattleManagementv3RoleTemplate::try_from(rt).unwrap();
        assert_eq!(
            result.metadata.as_ref().unwrap().name.as_deref(),
            Some("admin-template")
        );
        assert_eq!(result.display_name.as_deref(), Some("Admin"));
        assert_eq!(result.project_creator_default, Some(false));
    }

    #[test]
    fn test_iocattle_to_role_template_missing_metadata() {
        let mut io_rt = sample_iocattle_role_template();
        io_rt.metadata = None;
        let result = RoleTemplate::try_from(io_rt);
        assert!(result.is_err());
    }

    #[test]
    fn test_iocattle_to_role_template_missing_metadata_name() {
        let mut io_rt = sample_iocattle_role_template();
        io_rt.metadata.as_mut().unwrap().name = None;
        let result = RoleTemplate::try_from(io_rt);
        assert!(result.is_err());
    }

    #[test]
    fn test_round_trip_conversion() {
        let original = sample_role_template();
        let back_and_forth = RoleTemplate::try_from(
            IoCattleManagementv3RoleTemplate::try_from(original.clone()).unwrap(),
        )
        .unwrap();
        assert_eq!(original, back_and_forth);
    }


    #[test]
    fn test_equality_both_directions() {
        let rt = sample_role_template();
        let iort = sample_iocattle_role_template();

        // Forward comparison
        assert_eq!(iort, rt);

        // Reverse comparison
        assert_eq!(rt, iort);
    }

    #[test]
    fn test_inequality_on_field() {
        let rt = sample_role_template();
        let mut iort = sample_iocattle_role_template();

        // Change display name
        iort.display_name = Some("Changed".into());

        assert_ne!(rt, iort);
        assert_ne!(iort, rt);
    }

    #[test]
    fn test_missing_metadata_name() {
        let rt = sample_role_template();
        let mut iort = sample_iocattle_role_template();

        // Remove the name field
        if let Some(metadata) = iort.metadata.as_mut() {
            metadata.name = None;
        }

        assert_ne!(rt, iort);
        assert_ne!(iort, rt);
    }



}
