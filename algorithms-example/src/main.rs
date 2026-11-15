mod generate;

fn main() {
    // generate random values
    generate::generate_random_values();

    // generate random numbers within a range
    generate::generate_random_numbers_within_a_range();
    generate::generate_random_number_within_a_range_uniform();

    // generate random numbers with given distribution
    generate::generate_random_numbers_with_given_distribution();

    // generate random values of a custom type
    generate::generate_random_values_of_a_custom_type();

    // create random passwords from a set of alphnumeric characters
    generate::create_random_password_from_a_set_of_alphanumeric_characters();

    // create random password from a set of user-defined characters
    generate::create_random_password_from_a_set_of_user_defined_characters();
}
