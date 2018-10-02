defmodule Quicktype do
  def hello do
    :world
  end

  System.argv() |> IO.inspect()
  # File.read!(path)
end
