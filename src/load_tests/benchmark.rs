use ic_util::setup_test;

#[test]
fn stress_test_throughput() {
    setup_test!();
    
    for i in 0..10_000 {
        record_flow_data(i as f64 % 1000.0, Some("test_device".into()), None)
            .expect("Recording failed");
    }
    
    let stats = get_flow_statistics(None)
        .expect("Stats retrieval failed");
    
    assert!(stats.average > 0.0);
}