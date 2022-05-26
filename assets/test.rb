class Borculo end

class Borculo::Benz
  def hello_there(frenz: "brandon")
    puts "Nice to meet you there, #{frenz}; #{a}"
  end
end


puts Borculo::Benz.new.hello_there(frenz: "nice")
