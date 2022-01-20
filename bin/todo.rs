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
    // print args if verbose
    ctx.v_print("==FLAGS==");
    ctx.v_print(format!("Add      : {}", ctx.args.add));
    ctx.v_print(format!("Check    : {}", ctx.args.check));
    ctx.v_print(format!("New      : {}", ctx.args.new));
    ctx.v_print(format!("Quiet    : {}", ctx.args.quiet));
    ctx.v_print(format!("Show     : {}", ctx.args.show));
    ctx.v_print(format!("Verbose  : {}", ctx.args.verbose));
    ctx.v_print(format!("Uncheck  : {}", ctx.args.uncheck));
    ctx.v_print("==/FLAGS==");
    ctx.v_print("==RUN==");
    // create new list
    match ctx.args.mode {
        Mode::New(args) => {
            let list = ctx.args.list_path.clone().unwrap();
            ctx.v_print(format!("Creating new list \"{}\"", &list));
            ctx.check_path(PathExitCondition::Exists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let mut container = Container::create(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!(
                "Created new list \"{}\"", ctx.path.clone().to_str().unwrap()
            ));
        },
        Mode::Add(args) => {
            let list = ctx.args.list_path.clone().unwrap();
            let msg = ctx.args.message.clone().unwrap();
            ctx.v_print(
                format!(
                    "Creating new list-item in list \"{}\" with item-message \"{}\"",
                    list,
                    msg
                )
            );
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let item = if ctx.args.input_item.is_some() {
                ctx.args.input_item.clone().unwrap()
            } else {
                Vec::new()
            };
            container.add_item(ctx.args.item_type.clone(), &mut item.clone(), msg);
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
        },
        Mode::Check(args) => {
            let list = ctx.args.list_path.clone().unwrap();
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let item = ctx.args.input_item.clone().unwrap();
            ctx.v_print(format!("Checking item \"{}\" from list \"{}\"", get_printable_coords(&item), list));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.check_at(&mut item.clone());
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Checked from list \"{}\"", list));
        },
        Mode::Uncheck(args) => {
            let list = ctx.args.list_path.clone().unwrap();
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let item = ctx.args.input_item.clone().unwrap();
            ctx.v_print(format!(
                "Unchecking item \"{}\" from list \"{}\"",
                get_printable_coords(&item), list
            ));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.uncheck_at(&mut item.clone());
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Unchecked from list \"{}\"", list));
        },
        Mode::Edit(args) => {
            let list = ctx.args.list_path.clone().unwrap();
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let item = ctx.args.input_item.clone().unwrap();
            let message = ctx.args.message.clone().unwrap();
            ctx.v_print(format!(
                "Editing item \"{}\" from list \"{}\"",
                get_printable_coords(&item), list
            ));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.edit_at(&mut item.clone(), message);
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Edited item from list \"{}\"", list));
        },
        Mode::Remove(args) => {
            let list = ctx.args.list_path.clone().unwrap();
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let mut item = ctx.args.input_item.clone().unwrap();
            ctx.v_print(format!(
                "Removing item \"{}\" from list \"{}\"",
                get_printable_coords(&item), list
            ));
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.remove_at(&mut item);
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Removed item from list \"{}\"", list));
        },
        Mode::Move(args) => {
            let list = ctx.args.list_path.clone().unwrap();
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let mut item_loc = ctx.args.input_item.clone().unwrap();
            let mut end_loc = match ctx.args.output_item.clone() {
                Some(i) => {
                    ctx.v_print(format!(
                        concat!(
                            "Moving item in list \"{}\" from position ",
                            "\"{}\" to position \"{}\""
                        ),
                        list,
                        get_printable_coords(&item_loc),
                        get_printable_coords(&i),
                    ));
                    i
                },
                None => {
                    ctx.v_print(format!(
                        concat!(
                            "Moving item in list \"{}\" from position ",
                            "\"{}\" to top-level"
                        ),
                        list,
                        get_printable_coords(&item_loc),
                    ));
                    Vec::new()
                },
            };
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            container.move_from_to(&mut item_loc, &mut end_loc);
            container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print(format!("Moved item"));
        },
        Mode::Show(args) => {
            ctx.check_path(PathExitCondition::NotExists)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            ctx.v_print("==FILE==");
            let mut container = Container::load(&mut ctx)
                .unwrap_or_else(|e| safe_exit(&mut ctx, e));
            let mut content = String::new();
            container.print(&mut content, &ctx.args.print_which);
            ctx.print(content);
            ctx.v_print("==/FILE==");
            if args.status {
                ctx.check_path(PathExitCondition::NotExists)
                    .unwrap_or_else(|e| safe_exit(&mut ctx, e));
                ctx.v_print("==STATUS==");
                let mut content = String::new();
                let mut container = Container::load(&mut ctx)
                    .unwrap_or_else(|e| safe_exit(&mut ctx, e));
                container.status(&mut content, &ctx.args.print_which);
                ctx.print(content);
                ctx.v_print("==/STATUS==");
            }
        },
    }
    safe_exit(&mut ctx, ExitCode::Success);
}
