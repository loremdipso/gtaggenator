#!/usr/bin/env ruby
require 'json'

def main(args)
	rv = {}
	rv["filename"] = args[0]
	puts rv.to_json
end


main(ARGV.dup)

