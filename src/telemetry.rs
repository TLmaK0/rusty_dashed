#[macro_export]
macro_rules! telemetry {
    ( $graph_id: expr, $value: expr ) => {
        rusty_dashed::Server::send_message(format!("{}({})", $graph_id, $value));
    };
}
