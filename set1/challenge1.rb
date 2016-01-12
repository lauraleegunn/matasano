#!/usr/bin/env ruby
require 'base64'

string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"

# easy implementation - this one uses the standard 
# library functions available. better than reinventing
# the wheel!
module Easy
  def self.hex_to_raw h
    [h].pack('H*')
  end

  def self.raw_to_base64 r
    Base64.strict_encode64 r
  end

  def self.hex_to_base64 h
    raw_to_base64(hex_to_raw(h))
  end
end


module Hard
  def self.hex_to_raw h
    h.scan(/../).map do |byte|
      byte.hex.chr
    end.join
  end

  def self.hex_to_base64 h
    Easy.raw_to_base64(hex_to_raw(h))
  end
end


puts Easy.hex_to_base64(string)
puts Hard.hex_to_base64(string)
