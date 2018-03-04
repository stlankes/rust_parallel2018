default: .bundle slides.adoc
	bundle exec asciidoctor-revealjs -a revealjsdir=https://cdnjs.cloudflare.com/ajax/libs/reveal.js/3.6.0 slides.adoc

.bundle: Gemfile
	bundle config --local github.https true
	bundle --path=.bundle/gems --binstubs=.bundle/.bin

clean:
	rm -f slides.html
