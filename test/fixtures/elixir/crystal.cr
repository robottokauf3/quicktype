# Example code that deserializes and serializes the model:
#
# require "json"
#
# class Location
#   include JSON::Serializable
#
#   @[JSON::Field(key: "lat")]
#   property latitude : Float64
#
#   @[JSON::Field(key: "lng")]
#   property longitude : Float64
# end
#
# class House
#   include JSON::Serializable
#   property address : String
#   property location : Location?
# end
#
# house = House.from_json(%({"address": "Crystal Road 1234", "location": {"lat": 12.3, "lng": 34.5}}))
# house.address  # => "Crystal Road 1234"
# house.location # => #<Location:0x10cd93d80 @latitude=12.3, @longitude=34.5>

require "json"

class Pokedex
  include JSON::Serializable

  property pokemon : Array(Pokemon)
end

class Pokemon
  include JSON::Serializable

  property id : Int32

  property num : String

  property name : String

  property img : String

  @[JSON::Field(key: "type")]
  property pokemon_type : Array(String)

  property height : String

  property weight : String

  property candy : String

  property candy_count : Int32?

  property egg : String

  property spawn_chance : Float64

  property avg_spawns : Float64

  property spawn_time : String

  property multipliers : Array(Float64)?

  property weaknesses : Array(String)

  property next_evolution : Array(Evolution)?

  property prev_evolution : Array(Evolution)?
end

class Evolution
  include JSON::Serializable

  property num : String

  property name : String
end
