require 'marc'
require 'change_the_subject'
require 'byebug'
require 'change_the_subject_rs'
require 'benchmark/ips'
require 'erb' # required for the original change_the_subject implementation, but not declared in its gemspec

RubyVM::YJIT.enable

headings = File.readlines('subject-headings')

Benchmark.ips do |x|
  x.report('ruby') { ChangeTheSubject.fix(subject_terms: headings) }
  x.report('rust') { ChangeTheSubjectRS.fix(subject_terms: headings) }
  x.compare!
end


# How I made the subject-headings file
#outfile = File.open('subject-headings', 'w')
#reader = MARC::XMLReader.new("alma_dump.xml")
#headings = reader.reduce([]) do |acc, record|
# acc + record.select { it.tag == '650' }.map { |f| f.subfields.map(&:value).join('—') }
#end.sort.each { outfile.puts it }

