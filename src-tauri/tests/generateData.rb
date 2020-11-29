#!/usr/bin/env ruby
require 'fileutils'

def main(args)
	base = File.join("tmp", "test")
	num_dirs = 1
	## like 3s
	#num_files = 1_000_000
	num_files = 10000

	FileUtils.rm_rf(base)
	makeIfNotExists(base)
	Dir.chdir(base) do
		num_dirs.times do |i|
			dir = i.to_s
			makeIfNotExists(dir)
			Dir.chdir(dir) do
				num_files.times do |j|
					filename = "d_%03d-f_%03d.txt" % [i, j]
					createFile(filename, num_files*i + j)
				end
			end
		end
	end
end

def makeIfNotExists(dir)
	FileUtils.mkdir_p(dir) if !Dir.exists?(dir)
end

def createFile(filename, size)
	#contents = "A" * size
	contents = "A"
	File.open(filename, 'w') {|f| f.print(contents)}
end

main(ARGV.dup)

