use todo::Container;
use todo::Ctx;
use todo::ExitCode;
use todo::Mode;
use todo::PathExitCondition;
use todo::get_printable_coords;
fn safe_exit(ctx: &mut Ctx, e: ExitCode) -> ! {
    ctx.flush(&e);
    std::process::exit(e.into());
}
fn main() {
    let mut ctx = Ctx::init().unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(e.into());
    });
    ctx.v_print("==ARGS==");
    ctx.v_print(&format!("Mode: {}", &ctx.args.mode));
    ctx.v_print(&format!("Path: {}", &ctx.args.list_path));
    ctx.v_print("==/ARGS==");
    ctx.v_print("==RUN==");
    // create new list
    match ctx.args.mode.clone() {
        Mode::New => {
            ctx.v_print(format!("Creating new list \"{}\"", &ctx.args.list_path));
            ctx.check_path(PathExitCondition::Exists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let mut container = Container::create(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!(
                "Created new list \"{}\"", ctx.path.clone().to_str().unwrap()
            ));
        },
        Mode::Add(mut args) => {
            ctx.v_print(
                format!(
                    "Creating new list-item in list \"{}\" with item-message \"{}\"",
                    &ctx.args.list_path,
                    &args.item_message,
                )
            );
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.add_item(
                args.item_type,
                &mut args.item_nest_location,
                args.item_message,
                );
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
        },
        Mode::Check(mut args) => {
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!(
                "Checking item \"{}\" from list \"{}\"",
                get_printable_coords(&args.item_location),
                &ctx.args.list_path
            ));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.check_at(&mut args.item_location);
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Checked from list \"{}\"", &ctx.args.list_path));
        },
        Mode::Uncheck(mut args) => {
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!(
                "Unchecking item \"{}\" from list \"{}\"",
                get_printable_coords(&args.item_location), ctx.args.list_path
            ));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.uncheck_at(&mut args.item_location);
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Unchecked from list \"{}\"", ctx.args.list_path));
        },
        Mode::Edit(mut args) => {
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!(
                "Editing item \"{}\" from list \"{}\"",
                get_printable_coords(&args.item_location), &ctx.args.list_path
            ));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.edit_at(&mut args.item_location, args.item_message);
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Edited item from list \"{}\"", ctx.args.list_path));
        },
        Mode::Remove(mut args) => {
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!(
                "Removing item \"{}\" from list \"{}\"",
                get_printable_coords(&args.item_location),
                &ctx.args.list_path,
            ));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.remove_at(&mut args.item_location);
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Removed item from list \"{}\"", &ctx.args.list_path));
        },
        Mode::Move(mut args) => {
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.move_from_to(
                &mut args.item_location,
                &mut args.output_location,
            );
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Moved item"));
        },
        Mode::Show(args) => {
            let print_which = args.print_which;
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print("==FILE==");
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let mut content = String::new();
            container.print(&mut content, &print_which, args.plain);
            ctx.print(content);
            ctx.v_print("==/FILE==");
            if args.status {
                ctx.check_path(PathExitCondition::NotExists)
                    .unwrap_or_else(|e| safe_exit(&mut ctx, e));
                ctx.v_print("==STATUS==");
                let mut content = String::new();
                let mut container = Container::load(&mut ctx)
                    .unwrap_or_else(|e| safe_exit(&mut ctx, e));
                container.status(&mut content, &print_which);
                ctx.print(content);
                ctx.v_print("==/STATUS==");
            }
        },
    }
    safe_exit(&mut ctx, ExitCode::Success);
}
