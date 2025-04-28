use representation_app::run;

fn main() {
    pollster::block_on(run());
}
