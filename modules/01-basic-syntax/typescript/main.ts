import * as utils from './src/utils/utils';
import * as compound_types from './src/modules/compound_types';
import * as references from './src/modules/references';
import * as slices from './src/modules/slices';
import * as functions from './src/modules/functions';

utils.print_h1('Basic syntax');

// Compound types
utils.print_h2('Compound types');
compound_types.array_asignment_and_access();
compound_types.tuple_assignment_and_access();
utils.print_br();

// References
utils.print_h2('References');
references.references();
utils.print_br();

// Slices
utils.print_h2('Slices');
slices.array_slices();
slices.string_vs_str();
utils.print_br();

// Functions
utils.print_h2('Functions');
functions.fizzbuzz_to(10);
functions.test_methods_from_rectangle();
functions.test_overloading();
utils.print_br();
