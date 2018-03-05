default: .bundle slides.adoc
	bundle exec asciidoctor-revealjs -a revealjsdir=reveal.js-3.6.0/ slides.adoc

.bundle: Gemfile
	bundle config --local github.https true
	bundle --path=.bundle/gems --binstubs=.bundle/.bin

clean:
	rm -f slides.html

present: slides.adoc
	ruby -run -e httpd . -p 5000 -b 127.0.0.1 &
	open "http://localhost:5000/slides.html"
	wait
