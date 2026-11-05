# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = "change_the_subject_rs"
  spec.version = "0.1.0"
  spec.authors = ["Princeton University Library"]

  spec.summary = "Rust implementation of change_the_subject"
  spec.homepage = "https://github.com/pulibrary/change_the_subject_rs"
  spec.license = "BSD-3-Clause"
  spec.required_ruby_version = ">= 3.1.0"
  spec.required_rubygems_version = ">= 3.3.11"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/pulibrary/change_the_subject_rs"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  gemspec = File.basename(__FILE__)
  spec.files = IO.popen(%w[git ls-files -z], chdir: __dir__, err: IO::NULL) do |ls|
    ls.readlines("\x0", chomp: true).reject do |f|
      (f == gemspec) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git .circleci appveyor Gemfile])
    end
  end

  spec.require_paths = ["lib"]
  spec.extensions = ["ext/change_the_subject_ruby_bindings/extconf.rb"]

  spec.add_dependency "rb_sys", "~> 0.9.130"
end
