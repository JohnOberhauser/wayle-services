use tokio::sync::broadcast::Sender;

use crate::{Error, HyprlandEvent, Result};

pub(crate) fn handle_workspace(data: &str, hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
    hyprland_tx.send(HyprlandEvent::Workspace {
        name: data.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_workspace_v2(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let event_data = format!("{event}>>{data}");
    let Some((address, name)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data,
            field: "workspace_data",
            expected: "comma-separated address,name",
            value: data.to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::WorkspaceV2 {
        address: address.to_string(),
        name: name.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_create_workspace(
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    hyprland_tx.send(HyprlandEvent::CreateWorkspace {
        name: data.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_create_workspace_v2(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let event_data = format!("{event}>>{data}");
    let Some((address, name)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data,
            field: "workspace_data",
            expected: "comma-separated address,name",
            value: data.to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::CreateWorkspaceV2 {
        address: address.to_string(),
        name: name.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_destroy_workspace(
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    hyprland_tx.send(HyprlandEvent::DestroyWorkspace {
        name: data.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_destroy_workspace_v2(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let event_data = format!("{event}>>{data}");
    let Some((address, name)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data,
            field: "workspace_data",
            expected: "comma-separated address,name",
            value: data.to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::DestroyWorkspaceV2 {
        address: address.to_string(),
        name: name.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_move_workspace(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((name, monitor)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            field: "workspace_data",
            expected: "comma-separated name,monitor",
            value: data.to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::MoveWorkspace {
        name: name.to_string(),
        monitor: monitor.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_move_workspace_v2(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let event_data = format!("{event}>>{data}");
    let parts: Vec<&str> = data.split(',').collect();
    let [address, name, monitor] = parts.as_slice() else {
        return Err(Error::EventParseError {
            event_data,
            field: "workspace_data",
            expected: "3 comma-separated values (address,name,monitor)",
            value: data.to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::MoveWorkspaceV2 {
        address: address.to_string(),
        name: (*name).to_string(),
        monitor: (*monitor).to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_rename_workspace(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let event_data = format!("{event}>>{data}");
    let Some((address, new_name)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data,
            field: "workspace_data",
            expected: "comma-separated address,new_name",
            value: data.to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::RenameWorkspace {
        address: address.to_string(),
        new_name: new_name.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_active_special(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((workspace, monitor)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            field: "special_workspace_data",
            expected: "comma-separated workspace,monitor",
            value: data.to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::ActiveSpecial {
        workspace: workspace.to_string(),
        monitor: monitor.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_active_special_v2(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let event_data = format!("{event}>>{data}");
    let parts: Vec<&str> = data.split(',').collect();
    let [address, workspace, monitor] = parts.as_slice() else {
        return Err(Error::EventParseError {
            event_data,
            field: "special_workspace_data",
            expected: "3 comma-separated values (address,workspace,monitor)",
            value: data.to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::ActiveSpecialV2 {
        address: address.to_string(),
        workspace: (*workspace).to_string(),
        monitor: (*monitor).to_string(),
    })?;

    Ok(())
}
