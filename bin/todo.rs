use todo::Container;
use todo::ItemStatus;
use todo::Ctx;
use todo::ExitCode;
use todo::Mode;
use todo::PathExitCondition;
use todo::get_printable_coords;
use todo::ItemAction;
use todo::ItemActor;
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
            container.act_on_item_at(
                &mut args.item_nest_location,
                ItemAction::Add(args.item_type, args.item_message),
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
            container.act_on_item_at(
                &mut args.item_location,
                ItemAction::AlterStatus(ItemStatus::Complete),
            );
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Checked from list \"{}\"", &ctx.args.list_path));
        },
        Mode::Disable(mut args) => {
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!(
                "Disabling item \"{}\" from list \"{}\"",
                get_printable_coords(&args.item_location),
                &ctx.args.list_path
            ));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.act_on_item_at(
                &mut args.item_location,
                ItemAction::AlterStatus(ItemStatus::Disabled),
            );
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Checked from list \"{}\"", &ctx.args.list_path));
        },
        Mode::Hide(mut args) => {
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!(
                "Hiding item \"{}\" from list \"{}\"",
                get_printable_coords(&args.item_location),
                &ctx.args.list_path
            ));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.act_on_item_at(
                &mut args.item_location,
                ItemAction::AlterHidden(true),
            );
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Hid from list \"{}\"", &ctx.args.list_path));
        }
        Mode::Uncheck(mut args) => {
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!(
                "Unchecking item \"{}\" from list \"{}\"",
                get_printable_coords(&args.item_location), ctx.args.list_path
            ));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.act_on_item_at(
                &mut args.item_location,
                ItemAction::AlterStatus(ItemStatus::Incomplete),
            );
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Unchecked from list \"{}\"", ctx.args.list_path));
        },
        Mode::Unhide(mut args) => {
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!(
                "Unhiding item \"{}\" from list \"{}\"",
                get_printable_coords(&args.item_location),
                &ctx.args.list_path
            ));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.act_on_item_at(
                &mut args.item_location,
                ItemAction::AlterHidden(false),
            );
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Unhid from list \"{}\"", &ctx.args.list_path));
        }
        Mode::Edit(mut args) => {
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!(
                "Editing item \"{}\" from list \"{}\"",
                get_printable_coords(&args.item_location), &ctx.args.list_path
            ));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.act_on_item_at(
                &mut args.item_location,
                ItemAction::Edit(args.item_message),
            );
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
            container.act_on_item_at(
                &mut args.item_location,
                ItemAction::Remove,
            );
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Removed item from list \"{}\"", &ctx.args.list_path));
        },
        Mode::Move(mut args) => {
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let item = container.act_on_item_at(
                &mut args.item_location,
                ItemAction::Remove,
            ).unwrap();
            container.act_on_item_at(
                &mut args.output_location,
                ItemAction::Put(item),
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
            container.print(
                &mut content, &print_which, args.plain, args.level,
                args.display_hidden,
            );
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
