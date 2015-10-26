require 'openssl'
require 'securerandom'

class Camo
  def initialize(link)
    @link = link
  end

  def rewrite
    if configured? && !ssl_link?
      "#{host}/#{hex_digest}/#{hex_encode}"
    else
      @link
    end
  end

  def configured?
    host && key
  end

  private

  def host
    'http://localhost:3333'
  end

  def key
    '0x24FEEDFACEDEADBEEFCAFE'
  end

  def ssl_link?
    @link =~ /^https/
  end

  def hex_digest
    OpenSSL::HMAC.hexdigest(OpenSSL::Digest.new("sha1"), key, @link)
  end

  def hex_encode
    @link.to_enum(:each_byte).map { |byte| "%02x" % byte }.join
  end
end

p Camo.new('http://i.imgur.com/ieIYb0D.png').rewrite
