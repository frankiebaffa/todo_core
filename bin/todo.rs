use std::path::PathBuf;
use todo::Container;
use todo::Ctx;
use todo::ExitCode;
use todo::PathExitCondition;
use todo::get_printable_coords;
fn main() {
    let mut ctx = Ctx::init();
    if ctx.args.default_list {
        ctx.check_env();
    }
    // reverse vector so that pop works
    if ctx.args.item.is_some() {
        let mut item = ctx.args.item.unwrap();
        item.reverse();
        ctx.args.item = Some(item);
    }
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
    if ctx.args.new && ctx.args.list_path.is_some() {
        let list = ctx.args.list_path.clone().unwrap();
        ctx.v_print(format!("Creating new list \"{}\"", &list));
        let mut path = PathBuf::new();
        ctx.get_path(&mut path, "json", PathExitCondition::Exists);
        let mut container = Container::create(&mut ctx, &mut path);
        container.save(&mut ctx);
        ctx.v_print(format!("Created new list \"{}\"", &path.to_str().unwrap()));
    } else if ctx.args.new && ctx.args.list_path.is_none() {
        ctx.v_print("Missing name for list");
        ctx.exit(ExitCode::NoListName);
    }
    // add new list-item
    if ctx.args.add && ctx.args.list_path.is_some() && ctx.args.message.is_some() {
        let mut path = PathBuf::new();
        let list = ctx.args.list_path.clone().unwrap();
        let msg = ctx.args.message.clone().unwrap();
        ctx.v_print(
            format!(
                "Creating new list-item in list \"{}\" with item-message \"{}\"",
                list,
                msg
            )
        );
        ctx.get_path(&mut path, "json", PathExitCondition::NotExists);
        let mut container = Container::load(&mut ctx, &mut path);
        let item = if ctx.args.item.is_some() {
            ctx.args.item.clone().unwrap()
        } else {
            Vec::new()
        };
        container.add_item(&mut item.clone(), msg);
        container.save(&mut ctx);
    } else if ctx.args.add && ctx.args.list_path.is_none() {
        ctx.exit(ExitCode::NoListName);
    } else if ctx.args.add && ctx.args.message.is_none() {
        let mut path = PathBuf::new();
        ctx.get_path(&mut path, "md", PathExitCondition::Ignore);
        ctx.exit(ExitCode::NoListItemMessage(&mut path));
    }
    // check list item
    if ctx.args.check && ctx.args.list_path.is_some() && ctx.args.item.is_some() {
        let mut path = PathBuf::new();
        let list = ctx.args.list_path.clone().unwrap();
        ctx.get_path(&mut path, "json", PathExitCondition::NotExists);
        let item = ctx.args.item.clone().unwrap();
        ctx.v_print(format!("Checking item \"{}\" from list \"{}\"", get_printable_coords(&item), list));
        let mut container = Container::load(&mut ctx, &mut path);
        container.check_at(&mut item.clone());
        container.save(&mut ctx);
        ctx.v_print(format!("Checked from list \"{}\"", list));
    } else if ctx.args.check && ctx.args.list_path.is_none() {
        ctx.exit(ExitCode::NoListName);
    } else if ctx.args.check && ctx.args.item.is_none() {
        let mut path = PathBuf::new();
        ctx.get_path(&mut path, "md", PathExitCondition::Ignore);
        ctx.exit(ExitCode::NoListItemNumber(&mut path));
    }
    if ctx.args.uncheck && ctx.args.list_path.is_some() && ctx.args.item.is_some() {
        let mut path = PathBuf::new();
        let list = ctx.args.list_path.clone().unwrap();
        ctx.get_path(&mut path, "json", PathExitCondition::NotExists);
        let item = ctx.args.item.clone().unwrap();
        ctx.v_print(format!("Unchecking item \"{}\" from list \"{}\"", get_printable_coords(&item), list));
        let mut container = Container::load(&mut ctx, &mut path);
        container.uncheck_at(&mut item.clone());
        container.save(&mut ctx);
        ctx.v_print(format!("Unchecked from list \"{}\"", list));
    } else if ctx.args.uncheck && ctx.args.list_path.is_none() {
        ctx.exit(ExitCode::NoListName);
    } else if ctx.args.uncheck && ctx.args.item.is_none() {
        let mut path = PathBuf::new();
        ctx.get_path(&mut path, "md", PathExitCondition::Ignore);
        ctx.exit(ExitCode::NoListItemNumber(&mut path));
    }
    if ctx.args.show && ctx.args.list_path.is_some() {
        let mut path = PathBuf::new();
        ctx.get_path(&mut path, "json", PathExitCondition::NotExists);
        ctx.v_print("==FILE==");
        let mut content = String::new();
        let mut container = Container::load(&mut ctx, &mut path);
        container.printable(&mut content);
        ctx.print(content);
        ctx.v_print("==/FILE==");
    }
    ctx.exit(ExitCode::Success);
}
