class Borculo end

class Borculo::Benz
  def hello_there(frenz:)
    puts "Nice to meet you there, #{}"
  end
end


puts Borculo::Benz.new.hello_there(frenz: "nice")
