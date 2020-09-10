use goose::prelude::*;

async fn loadtest_uniques(user: &GooseUser) -> GooseTaskResult {
    let _goose = user.get("/uniques/").await?;
    Ok(())
}

fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_taskset(
            taskset!("LoadtestTasks")
                .register_task(task!(loadtest_uniques)),
        )
        .execute()?
        .print();
    Ok(())
}
