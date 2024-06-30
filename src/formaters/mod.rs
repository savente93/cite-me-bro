pub mod html;
pub mod markdown;
pub mod plain;

// Some rudimentary benchmarking has shown that mutating inplace is the fastest
// for benchmarks see https://github.com/savente93/str_manip_bench
pub trait Formatter {
    fn italics(input: &mut String);
    fn bold(input: &mut String);
    fn hyperlink(input: &mut String);
}
