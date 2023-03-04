mod modules; // this declares a root module called `modules`
mod utils;

fn main() {
    let print_h1 = utils::print_h1;
    let print_h2 = utils::print_h2;
    let print_br = utils::print_br;

    print_h1("Basic syntax");

    // Compound types
    print_h2("Compund types");
    modules::compound_types::array_asignment_and_access();
    modules::compound_types::tuple_assignment_and_access();
    print_br();

    // References
    print_h2("References");
    modules::references::references();
    print_br();

    // Slices
    print_h2("Slices");
    modules::slices::array_slices();
    modules::slices::string_vs_str();
    print_br();

    // Functions
    print_h2("Functions");
    modules::functions::fizzbuzz_to(10);
    modules::functions::test_methods_from_rectangle();
    modules::functions::test_overloading();
    print_br();
}
