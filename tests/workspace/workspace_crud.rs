use client_api_test_util::generate_unique_registered_user_client;
use database_entity::dto::AFRole;
use shared_entity::dto::workspace_dto::CreateWorkspaceMember;
use shared_entity::dto::workspace_dto::CreateWorkspaceParam;
use shared_entity::dto::workspace_dto::PatchWorkspaceParam;

#[tokio::test]
async fn add_and_delete_workspace_for_user() {
  let (c, _user) = generate_unique_registered_user_client().await;
  let workspaces = c.get_workspaces().await.unwrap();
  assert_eq!(workspaces.0.len(), 1);
  let newly_added_workspace = c
    .create_workspace(CreateWorkspaceParam {
      workspace_name: Some("my_workspace".to_string()),
    })
    .await
    .unwrap();
  let workspaces = c.get_workspaces().await.unwrap();
  assert_eq!(workspaces.0.len(), 2);

  let _ = workspaces
    .0
    .iter()
    .find(|w| {
      w.workspace_name == "my_workspace" && w.workspace_id == newly_added_workspace.workspace_id
    })
    .unwrap();

  c.delete_workspace(&newly_added_workspace.workspace_id.to_string())
    .await
    .unwrap();
  let workspaces = c.get_workspaces().await.unwrap();
  assert_eq!(workspaces.0.len(), 1);
}

#[tokio::test]
async fn add_and_delete_workspace_for_non_owner_user() {
  let (member, member_user) = generate_unique_registered_user_client().await;

  // Owner added member to workspace
  let (owner, _user) = generate_unique_registered_user_client().await;
  let owner_workspace = owner
    .create_workspace(CreateWorkspaceParam {
      workspace_name: Some("owner_workspace".to_string()),
    })
    .await
    .unwrap();

  owner
    .add_workspace_members(
      owner_workspace.workspace_id.to_string(),
      vec![CreateWorkspaceMember {
        email: member_user.email.clone(),
        role: AFRole::Member,
      }],
    )
    .await
    .unwrap();

  // Member should have 2 workspaces
  let member_workspaces = member.get_workspaces().await.unwrap();
  assert_eq!(member_workspaces.0.len(), 2);

  owner
    .remove_workspace_members(
      owner_workspace.workspace_id.to_string(),
      vec![member_user.email],
    )
    .await
    .unwrap();

  // Member should have 1 workspaces, because owner removed him
  let member_workspaces = member.get_workspaces().await.unwrap();
  assert_eq!(member_workspaces.0.len(), 1);
}

#[tokio::test]
async fn test_workspace_rename_and_icon_change() {
  let (c, _user) = generate_unique_registered_user_client().await;
  let workspace_id = c
    .get_workspaces()
    .await
    .unwrap()
    .0
    .first()
    .unwrap()
    .workspace_id;
  let desired_new_name = "tom's workspace";

  {
    c.patch_workspace(PatchWorkspaceParam {
      workspace_id,
      workspace_name: Some(desired_new_name.to_string()),
      ..Default::default()
    })
    .await
    .expect("Failed to rename workspace");

    let workspaces = c.get_workspaces().await.expect("Failed to get workspaces");
    let actual_new_name = &workspaces
      .0
      .first()
      .expect("No workspace found")
      .workspace_name;
    assert_eq!(actual_new_name, desired_new_name);
  }

  {
    c.patch_workspace(PatchWorkspaceParam {
      workspace_id,
      workspace_name: None,
      ..Default::default()
    })
    .await
    .expect("Failed to rename workspace");
    let workspaces = c.get_workspaces().await.expect("Failed to get workspaces");
    let actual_new_name = &workspaces
      .0
      .first()
      .expect("No workspace found")
      .workspace_name;
    assert_eq!(actual_new_name, desired_new_name);
  }
  {
    c.patch_workspace(PatchWorkspaceParam {
      workspace_id,
      workspace_icon: Some("icon123".to_string()),
      ..Default::default()
    })
    .await
    .expect("Failed to change icon");
    let workspaces = c.get_workspaces().await.expect("Failed to get workspaces");
    let icon = &workspaces.0.first().expect("No workspace found").icon;
    assert_eq!(icon, "icon123");
  }
  {
    c.patch_workspace(PatchWorkspaceParam {
      workspace_id,
      workspace_name: Some("new_name456".to_string()),
      workspace_icon: Some("new_icon456".to_string()),
    })
    .await
    .expect("Failed to change icon");
    let workspaces = c.get_workspaces().await.expect("Failed to get workspaces");
    let workspace = workspaces.0.first().expect("No workspace found");

    let icon = workspace.icon.as_str();
    let name = workspace.workspace_name.as_str();
    assert_eq!(icon, "new_icon456");
    assert_eq!(name, "new_name456");
  }
}
