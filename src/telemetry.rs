#[macro_export]
macro_rules! telemetry {
    ( $graph_id: expr, $value: expr ) => {
        rusty_dashed::WsServer::send_message(format!("{}({})", $graph_id, $value));
    };
}
