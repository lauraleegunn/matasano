#!/usr/bin/env ruby
# I ripped some ASCII character frequency data from
# http://fitaly.com/board/domper3/posts/136.html.
# It's pretty malformed, so this script will fix what
# it can.

# read the data
letter_frequency_data = File.open("letter_frequency_data.txt").read

# split into atomic pieces
data = letter_frequency_data.split("\n").map{|n| n.split(" ")}

# fix some edge cases
data[0][2] = data[0][2].slice(1,20)
data[0].insert(1, " ")
data[0].insert(3, "(")

# extract useful information
data = data.map{|x| [x[1], (10*x[4].slice(0, 4).to_f).round]}

# remove anything useless
data = data.reject{|x| x[1] == 0}

# escape parentheses
data = data.map{|x| x[0]=="'" && ["\\'", x[1]] || x}

# turn into rust match expressions
data = data.map{|x| "'#{x[0]}' => #{x[1]},"}

puts data.join("\n")
