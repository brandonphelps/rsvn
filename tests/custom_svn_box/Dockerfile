FROM elleflorio/svn-server

ADD configure.sh /
RUN /bin/sh /configure.sh 
ADD svnserve.conf /home/svn/Test/conf/svnserve.conf
ADD passwd /home/svn/Test/conf/passwd