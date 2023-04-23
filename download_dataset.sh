#!/bin/sh
wget http://svn.code.sf.net/p/cmusphinx/code/trunk/cmudict/cmudict-0.7b
wget http://svn.code.sf.net/p/cmusphinx/code/trunk/cmudict/scripts/make_baseform.pl
perl make_baseform.pl cmudict-0.7b cmudict-0.7b_baseform