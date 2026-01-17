use niri_ipc::Action::{FocusWorkspace, MoveWorkspaceToMonitor};
use niri_ipc::socket::Socket;
use niri_ipc::{Request, Response, Workspace, WorkspaceReferenceArg};

fn main() {
    let mut sock = Socket::connect().expect("Connection failed");
    let workspaces = get_workspaces(&mut sock);
    let current_state: Vec<(&str, u64, bool)> = workspaces
        .iter()
        .filter_map(|workspace| match (&workspace.output, workspace.is_active) {
            (Some(output), true) => Some((output.as_str(), workspace.id, workspace.is_focused)),
            _ => None,
        })
        .collect();
    let mut workspace_to_focus = None;
    for (a, b) in current_state
        .iter()
        .zip(current_state.iter().cycle().skip(1))
    {
        let (new_output, old_output) = (a.0, b.0);
        let (new_workspace_id, to_focus) = (b.1, !b.2);
        move_workspace_to_monitor(&mut sock, new_workspace_id, new_output);
        // Focus workspace to keep it active
        focus_workspace(&mut sock, new_workspace_id);
        if to_focus {
            workspace_to_focus = Some(new_workspace_id);
        }
        println!("Moved workspace #{new_workspace_id} from {old_output} to {new_output}");
    }
    // Focus workspace on the last focused output
    if let Some(workspace_id) = workspace_to_focus {
        focus_workspace(&mut sock, workspace_id);
    }
}

macro_rules! send_request {
    ($sock:expr, $req:expr) => {{
        match send_request($sock, $req) {
            Response::Handled => {}
            rep => panic!("Got an unexpected response: {:?}", rep),
        }
    }};
    ($sock:expr, $req:expr, $pattern:pat => $var:ident) => {{
        match send_request($sock, $req) {
            $pattern => $var,
            rep => panic!("Got an unexpected response: {:?}", rep),
        }
    }};
}

fn focus_workspace(sock: &mut Socket, workspace_id: u64) {
    let reference = WorkspaceReferenceArg::Id(workspace_id);
    send_request!(sock, Request::Action(FocusWorkspace { reference }))
}

fn get_workspaces(sock: &mut Socket) -> Vec<Workspace> {
    send_request!(sock, Request::Workspaces, Response::Workspaces(x) => x)
}

fn move_workspace_to_monitor(sock: &mut Socket, workspace_id: u64, output: impl Into<String>) {
    let output = output.into();
    let reference = Some(WorkspaceReferenceArg::Id(workspace_id));
    send_request!(
        sock,
        Request::Action(MoveWorkspaceToMonitor { output, reference })
    )
}

fn send_request(sock: &mut Socket, req: Request) -> Response {
    match sock.send(req) {
        Ok(Ok(rep)) => rep,
        Ok(Err(e)) => panic!("Niri error: {}", e),
        Err(e) => panic!("Request failed: {}", e),
    }
}
