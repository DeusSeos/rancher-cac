use serde::{Deserialize, Serialize};

use rancher_client::apis::{configuration::Configuration, Error, ResponseContent};
use reqwest::StatusCode;

use rancher_client::{
    apis::management_cattle_io_v3_api::{
        list_management_cattle_io_v3_namespaced_project_role_template_binding,
        list_management_cattle_io_v3_project_role_template_binding_for_all_namespaces,
        ListManagementCattleIoV3NamespacedProjectRoleTemplateBindingError,
        ListManagementCattleIoV3ProjectRoleTemplateBindingForAllNamespacesError,
    },
    models::{
        IoCattleManagementv3ProjectRoleTemplateBinding,
        IoCattleManagementv3ProjectRoleTemplateBindingList,
        IoK8sApimachineryPkgApisMetaV1ObjectMeta,
    },
};


pub const PRTB_EXCLUDE_PATHS: &[&str] = &[
    "metadata.creationTimestamp",
    "metadata.finalizers",
    "metadata.generateName",
    "metadata.generation",
    "metadata.managedFields",
    "metadata.resourceVersion",
    "metadata.selfLink",
    "metadata.uid",
];

/// Get all project role template bindings from an endpoint using the provided configuration
///
/// # Arguments
///
/// * `configuration` - The configuration to use for the request
///
/// # Returns
///
/// * `IoCattleManagementv3ProjectRoleTemplateBindingList` - The list of project role template bindings
///
/// # Errors
///
/// * `Error<ListManagementCattleIoV3ProjectRoleTemplateBindingForAllNamespacesError>` - The error that occurred while trying to get the bindings
///
#[async_backtrace::framed]
pub async fn get_project_role_template_bindings(
    configuration: &Configuration,
    field_selector: Option<&str>,
    label_selector: Option<&str>,
    limit: Option<i32>,
    resource_version: Option<&str>,
    resource_version_match: Option<&str>,
    continue_: Option<&str>,
) -> Result<
    IoCattleManagementv3ProjectRoleTemplateBindingList,
    Error<ListManagementCattleIoV3ProjectRoleTemplateBindingForAllNamespacesError>,
> {
    let result = list_management_cattle_io_v3_project_role_template_binding_for_all_namespaces(
        configuration,
        None,
        continue_,
        field_selector,
        label_selector,
        limit,
        None,
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
                    // Try to deserialize the content into IoCattleManagementv3ProjectRoleTemplateBindingList (Status200 case)
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
                                entity: Some(ListManagementCattleIoV3ProjectRoleTemplateBindingForAllNamespacesError::UnknownValue(
                                    unknown_data,
                                )),
                            }))
                        }
                        Err(deserialize_err) => Err(Error::Serde(deserialize_err)),
                    }
                }
            }
        }
    }
}

/// Get all project role template bindings from a namespace using the provided configuration
///
/// # Arguments
///
/// * `configuration` - The configuration to use for the request
/// * `cluster_id` - The ID of the cluster (namespace) to get the project role template bindings for
///
/// # Returns
///
/// * `IoCattleManagementv3ProjectRoleTemplateBindingList` - The list of project role template bindings
///
/// # Errors
///
/// * `Error<ListManagementCattleIoV3ProjectRoleTemplateBindingForAllNamespacesError>` - The error that occurred while trying to get the bindings
#[async_backtrace::framed]
pub async fn get_namespaced_project_role_template_bindings(
    configuration: &Configuration,
    project_id: &str,
    field_selector: Option<&str>,
    label_selector: Option<&str>,
    limit: Option<i32>,
    resource_version: Option<&str>,
    resource_version_match: Option<&str>,
    continue_: Option<&str>,
) -> Result<
    IoCattleManagementv3ProjectRoleTemplateBindingList,
    Error<ListManagementCattleIoV3NamespacedProjectRoleTemplateBindingError>,
> {
    let result = list_management_cattle_io_v3_namespaced_project_role_template_binding(
        configuration,
        project_id,
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
                    // Try to deserialize the content into IoCattleManagementv3ProjectRoleTemplateBindingList (Status200 case)
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
                                entity: Some(ListManagementCattleIoV3NamespacedProjectRoleTemplateBindingError::UnknownValue(
                                    unknown_data,
                                )),
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
pub struct ProjectRoleTemplateBinding {
    // annotations: Option<std::collections::HashMap<String, String>>,
    /// Annotations applied to the project role template binding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_principal_name: Option<String>,
    /// The name of the project role template binding (typically the Kubernetes metadata.name).
    pub id: String,

    /// Labels applied to the project role template binding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,

    /// the project (namespace) the project role template exists in
    pub namespace: String,

    /// The name of the project the project role template is bound to (cluster-id:project-id)
    pub project_name: String,

    pub role_template_name: String,

    /// An opaque value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server. They may only be valid for a particular resource or set of resources.  Populated by the system. Read-only. Value must be treated as opaque by clients and . More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,

    /// The UID of the project. This cannot be changed. Rancher will set this value when the project is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}

impl ProjectRoleTemplateBinding {
    pub fn new(
        annotations: Option<std::collections::HashMap<String, String>>,
        group_name: Option<String>,
        group_principal_name: Option<String>,
        id: String,
        labels: Option<std::collections::HashMap<String, String>>,
        namespace: String,
        project_name: String,
        uid: Option<String>,
        role_template_name: String,
        resource_version: Option<String>,
        service_account: Option<String>,
        user_name: Option<String>,
        user_principal_name: Option<String>,
    ) -> Self {
        ProjectRoleTemplateBinding {
            annotations,
            group_name,
            group_principal_name,
            id,
            labels,
            namespace,
            project_name,
            role_template_name,
            resource_version,
            service_account,
            uid,
            user_name,
            user_principal_name,
        }
    }
}

impl TryFrom<IoCattleManagementv3ProjectRoleTemplateBinding> for ProjectRoleTemplateBinding {
    type Error = &'static str;

    fn try_from(
        value: IoCattleManagementv3ProjectRoleTemplateBinding,
    ) -> Result<Self, Self::Error> {
        let metadata: IoK8sApimachineryPkgApisMetaV1ObjectMeta =
            value.metadata.ok_or("missing metadata")?;

        let id = metadata.name.ok_or("missing name")?;

        // Extract the fields from the IoCattleManagementv3ProjectRoleTemplateBinding
        // and create a new ProjectRoleTemplateBinding instance
        let group_name = value.group_name;
        let group_principal_name = value.group_principal_name;
        let project_name = value.project_name;
        let role_template_name = value.role_template_name;
        let service_account = value.service_account;
        let user_name = value.user_name;
        let user_principal_name = value.user_principal_name;
        let annotations = metadata.annotations.map(|a| {
            a.into_iter()
                .collect::<std::collections::HashMap<String, String>>()
        });

        let labels = metadata.labels.map(|a| {
            a.into_iter()
                .collect::<std::collections::HashMap<String, String>>()
        });
        let namespace = metadata.namespace.unwrap_or_default();
        let resource_version = metadata.resource_version;
        let uid = metadata.uid;

        Ok(ProjectRoleTemplateBinding {
            id,
            group_name,
            group_principal_name,
            project_name,
            role_template_name,
            service_account,
            user_name,
            user_principal_name,
            annotations,
            labels,
            namespace,
            resource_version,
            uid,
        })
    }
}

impl TryFrom<ProjectRoleTemplateBinding> for IoCattleManagementv3ProjectRoleTemplateBinding {
    type Error = &'static str;

    fn try_from(value: ProjectRoleTemplateBinding) -> Result<Self, Self::Error> {
        // Create a new IoCattleManagementv3ProjectRoleTemplateBinding instance
        let metadata = IoK8sApimachineryPkgApisMetaV1ObjectMeta {
            annotations: value.annotations,
            labels: value.labels,
            namespace: Some(value.namespace),
            name: Some(value.id.clone()),
            ..Default::default()
        };

        Ok(IoCattleManagementv3ProjectRoleTemplateBinding {
            api_version: Some("management.cattle.io/v3".to_string()),
            group_name: value.group_name,
            group_principal_name: value.group_principal_name,
            kind: Some("ProjectRoleTemplateBinding".to_string()),
            metadata: Some(metadata),
            project_name: value.project_name,
            role_template_name: value.role_template_name,
            service_account: value.service_account,
            user_name: value.user_name,
            user_principal_name: value.user_principal_name,
        })
    }
}

impl PartialEq<ProjectRoleTemplateBinding> for IoCattleManagementv3ProjectRoleTemplateBinding {
    fn eq(&self, other: &ProjectRoleTemplateBinding) -> bool {
        let lhs = self.metadata.as_ref().and_then(|m| m.name.clone());
        let rhs = Some(other.id.clone());

        lhs == rhs
            && self.group_name == other.group_name
            && self.group_principal_name == other.group_principal_name
            && self.project_name == other.project_name
            && self.role_template_name == other.role_template_name
            && self.service_account == other.service_account
            && self.user_name == other.user_name
            && self.user_principal_name == other.user_principal_name
    }
}

impl PartialEq<IoCattleManagementv3ProjectRoleTemplateBinding> for ProjectRoleTemplateBinding {
    fn eq(&self, other: &IoCattleManagementv3ProjectRoleTemplateBinding) -> bool {
        // let lhs = Some(self.id.clone());
        // let rhs = other.metadata.as_ref().and_then(|m| m.name.clone());

        // lhs == rhs
        //     && self.group_name == other.group_name
        //     && self.group_principal_name == other.group_principal_name
        //     && self.project_name == other.project_name
        //     && self.role_template_name == other.role_template_name
        //     && self.service_account == other.service_account
        //     && self.user_name == other.user_name
        //     && self.user_principal_name == other.user_principal_name

        other == self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_binding() -> ProjectRoleTemplateBinding {
        ProjectRoleTemplateBinding {
            id: "binding-id".to_string(),
            group_name: Some("group1".to_string()),
            group_principal_name: Some("groupPrincipal".to_string()),
            project_name: "project-id".to_string(),
            role_template_name: "role-template".to_string(),
            service_account: Some("service-account".to_string()),
            user_name: Some("user1".to_string()),
            user_principal_name: Some("userPrincipal".to_string()),
            annotations: Some(std::collections::HashMap::new()),
            labels: Some(std::collections::HashMap::new()),
            namespace: "namespace-id".to_string(),
            resource_version: Some("resource-version".to_string()),
            uid: Some("uid".to_string()),
        }
    }

    fn sample_iocattle_binding() -> IoCattleManagementv3ProjectRoleTemplateBinding {
        IoCattleManagementv3ProjectRoleTemplateBinding {
            api_version: Some("management.cattle.io/v3".to_string()),
            kind: Some("ProjectRoleTemplateBinding".to_string()),
            metadata: Some(IoK8sApimachineryPkgApisMetaV1ObjectMeta {
                name: Some("binding-id".to_string()),
                ..Default::default()
            }),
            group_name: Some("group1".to_string()),
            group_principal_name: Some("groupPrincipal".to_string()),
            project_name: "project-id".to_string(),
            role_template_name: "role-template".to_string(),
            service_account: Some("service-account".to_string()),
            user_name: Some("user1".to_string()),
            user_principal_name: Some("userPrincipal".to_string()),
        }
    }

    #[test]
    fn test_equality_both_directions() {
        let a = sample_binding();
        let b = sample_iocattle_binding();

        assert_eq!(a, b);
        assert_eq!(b, a);
    }

    #[test]
    fn test_try_from_iocattle_to_binding() {
        let ioc = sample_iocattle_binding();
        let result = ProjectRoleTemplateBinding::try_from(ioc);
        assert!(result.is_ok());

        let binding = result.unwrap();
        assert_eq!(binding.id, "binding-id");
        assert_eq!(binding.group_name.as_deref(), Some("group1"));
    }

    #[test]
    fn test_try_from_binding_to_iocattle() {
        let binding = sample_binding();
        let result = IoCattleManagementv3ProjectRoleTemplateBinding::try_from(binding);
        assert!(result.is_ok());

        let ioc = result.unwrap();
        assert_eq!(ioc.metadata.unwrap().name, Some("binding-id".to_string()));
        assert_eq!(ioc.group_name.as_deref(), Some("group1"));
    }

    #[test]
    fn test_inequality_on_different_user() {
        let a = sample_binding();
        let mut b = sample_iocattle_binding();

        b.user_name = Some("other-user".to_string());

        assert_ne!(a, b);
        assert_ne!(b, a);
    }

    #[test]
    fn test_missing_metadata_name() {
        let mut b = sample_iocattle_binding();
        b.metadata.as_mut().unwrap().name = None;

        let result = ProjectRoleTemplateBinding::try_from(b);
        assert!(result.is_err());
    }

    #[test]
    fn test_inequality_on_missing_metadata_name_in_eq() {
        let a = sample_binding();
        let mut b = sample_iocattle_binding();
        b.metadata.as_mut().unwrap().name = None;

        assert_ne!(a, b);
        assert_ne!(b, a);
    }
}
