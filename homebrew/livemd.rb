class Livemd < Formula
  desc "Simple Markdown streaming tool for terminals"
  homepage "https://github.com/victoria-riley-barnett/livemd"
  url "https://github.com/victoria-riley-barnett/livemd/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256"
  license "MIT OR Apache-2.0"

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