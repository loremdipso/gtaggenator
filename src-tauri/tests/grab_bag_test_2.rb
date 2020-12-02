#!/usr/bin/ruby
require 'open3'

path = "./tmp/A.txt"
Open3.popen2("/home/madams/.cargo-target/release/taggenator", "grabbag") do |stdin, stdout, stderr|
	#"add" "./tmp/B.txt" key_$i value_$i --ignore-update

	bg = Thread.new{
		while true
			if stdout.eof?
				break
			end

			puts stdout.gets # grab some junk so rust doesn't complain
		end
	}

	10000.times do |i|
		stdin.puts("add \"#{path}\" key_#{i} value_#{i}")
	end


	stdin.close
	bg.join
	#sleep(5)
end

puts `/home/madams/.cargo-target/release/taggenator grabbag get_all "#{path}"`

