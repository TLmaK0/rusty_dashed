#[macro_export]
macro_rules! telemetry {
    ( $graph_id: expr, $throttle: expr, $value: expr ) => {
        #[cfg(feature = "telemetry")]
        {
            if $crate::random::<f64>() < $throttle{

                rusty_dashed::WsServer::send_message(format!("{}({})", $graph_id, $value));
            }
        }
    };
}
