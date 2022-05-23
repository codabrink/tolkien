class Borculo end

class Borculo::Benz
  def hello_there(frenz: nil, nice) puts "Nice to meet you there, #{frenz}" end
end


puts Borculo::Benz.new.hello_there(frenz: "nice")
