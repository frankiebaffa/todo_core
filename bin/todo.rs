use todo::Container;
use todo::Ctx;
use todo::ExitCode;
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
    match ctx.new_list_mode() {
        Ok(is) => {
            if is {
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
            }
        },
        Err(e) => safe_exit(&mut ctx, e),
    }
    // add new list-item
    match ctx.new_item_mode() {
        Ok(is) => {
            if is {
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
                let item = if ctx.args.item.is_some() {
                    ctx.args.item.clone().unwrap()
                } else {
                    Vec::new()
                };
                container.add_item(&mut item.clone(), msg);
                container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
            }
        },
        Err(e) => safe_exit(&mut ctx, e),
    }
    // check list item
    match ctx.check_item_mode() {
        Ok(is) => {
            if is {
                let list = ctx.args.list_path.clone().unwrap();
                ctx.check_path(PathExitCondition::NotExists)
                    .unwrap_or_else(|e| safe_exit(&mut ctx, e));
                let item = ctx.args.item.clone().unwrap();
                ctx.v_print(format!("Checking item \"{}\" from list \"{}\"", get_printable_coords(&item), list));
                let mut container = Container::load(&mut ctx)
                    .unwrap_or_else(|e| safe_exit(&mut ctx, e));
                container.check_at(&mut item.clone());
                container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
                ctx.v_print(format!("Checked from list \"{}\"", list));
            }
        },
        Err(e) => safe_exit(&mut ctx, e),
    }
    // uncheck list item
    match ctx.uncheck_item_mode() {
        Ok(is) => {
            if is {
                let list = ctx.args.list_path.clone().unwrap();
                ctx.check_path(PathExitCondition::NotExists)
                    .unwrap_or_else(|e| safe_exit(&mut ctx, e));
                let item = ctx.args.item.clone().unwrap();
                ctx.v_print(format!(
                    "Unchecking item \"{}\" from list \"{}\"",
                    get_printable_coords(&item), list
                ));
                let mut container = Container::load(&mut ctx)
                    .unwrap_or_else(|e| safe_exit(&mut ctx, e));
                container.uncheck_at(&mut item.clone());
                container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
                ctx.v_print(format!("Unchecked from list \"{}\"", list));
            }
        },
        Err(e) => safe_exit(&mut ctx, e),
    }
    // edit list item
    match ctx.edit_mode() {
        Ok(is) => {
            if is {
                let list = ctx.args.list_path.clone().unwrap();
                ctx.check_path(PathExitCondition::NotExists)
                    .unwrap_or_else(|e| safe_exit(&mut ctx, e));
                let item = ctx.args.item.clone().unwrap();
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
            }
        },
        Err(e) => safe_exit(&mut ctx, e),
    }
    // remove list item
    match ctx.remove_mode() {
        Ok(is) => {
            if is {
                let list = ctx.args.list_path.clone().unwrap();
                ctx.check_path(PathExitCondition::NotExists)
                    .unwrap_or_else(|e| safe_exit(&mut ctx, e));
                let item = ctx.args.item.clone().unwrap();
                ctx.v_print(format!(
                    "Removing item \"{}\" from list \"{}\"",
                    get_printable_coords(&item), list
                ));
                let mut container = Container::load(&mut ctx)
                    .unwrap_or_else(|e| safe_exit(&mut ctx, e));
                container.remove_at(&mut item.clone());
                container.save().unwrap_or_else(|e| safe_exit(&mut ctx, e));
                ctx.v_print(format!("Removed item from list \"{}\"", list));
            }
        },
        Err(e) => safe_exit(&mut ctx, e),
    }
    // show list
    match ctx.show_mode() {
        Ok(is) => {
            if is {
                ctx.check_path(PathExitCondition::NotExists)
                    .unwrap_or_else(|e| safe_exit(&mut ctx, e));
                ctx.v_print("==FILE==");
                let mut content = String::new();
                let mut container = Container::load(&mut ctx)
                    .unwrap_or_else(|e| safe_exit(&mut ctx, e));
                container.printable(&mut content);
                ctx.print(content);
                ctx.v_print("==/FILE==");
            }
        },
        Err(e) => safe_exit(&mut ctx, e),
    }
    safe_exit(&mut ctx, ExitCode::Success);
}
