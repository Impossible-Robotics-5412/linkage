use gauge::Gauge;

fn main() {
    let config = config::config().unwrap();
    let gauge = Gauge::new(config.gauge().port());
    gauge.start();
}
