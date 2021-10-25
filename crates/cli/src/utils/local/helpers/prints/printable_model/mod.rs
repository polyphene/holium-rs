//! Trait to implement to print formatted objects in a table format.

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
    fn table_print(objects: Vec<Self>) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.set_titles(Self::title_row());
        for o in objects {
            table.add_row(o.object_to_row());
        };
        table.printstd();
    }
}