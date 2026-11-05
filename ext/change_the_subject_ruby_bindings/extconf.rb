# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

# See https://github.com/oxidize-rb/rb-sys/blob/main/gem/lib/rb_sys/mkmf.rb
# to see documentation on how makefile is created.
create_rust_makefile("change_the_subject_rs")
