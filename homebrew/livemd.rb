class Livemd < Formula
  desc "Simple Markdown streaming tool for terminals"
  homepage "https://github.com/victoria-riley-barnett/livemd"
  url "https://github.com/victoria-riley-barnett/livemd/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "af5b188b117069b02e9aeaca954efb155d9a1c590a125b37c609a5ce038f16ab"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--root", prefix, "--path", "."
  end

  test do
    # Test basic functionality
    (testpath/"test.md").write("# Hello World\n\nThis is a test.")
    assert_match "Hello World", shell_output("#{bin}/livemd --file #{testpath}/test.md")
  end
end