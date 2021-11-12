//! Trait to implement to print formatted objects in a table format.

use console::style;
use prettytable::{Table, format, Row};

/// PrintableModel can be implemented to print objects in a table format
pub trait PrintableModel: Sized {
    /// Defines the default content of a table title row.
    ///
    /// # Example
    ///
    /// ```
    /// fn title_row() -> Row {
    ///     row![b->"NAME", "HANDLE"]
    /// }
    /// ```
    fn title_row() -> Row;

    /// Defines the default content of a table body row.
    ///
    /// # Example
    ///
    /// ```
    /// fn object_to_row(&self) -> Row {
    ///     row![b->self.name, self.handle]
    /// }
    /// ```
    fn object_to_row(&self) -> Row;

    /// Print a list of objects into a table.
    fn table_print(objects: Vec<&Self>) {
        if objects.len() < 1 {
            println!("{}", style("no object in the list").yellow());
            return
        }
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        table.set_titles(Self::title_row());
        for object in objects {
            table.add_row(object.object_to_row());
        };
        table.printstd();
    }
}