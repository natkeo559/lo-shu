%         *****************************************
%         *              rhumj_new.cls            *
%         *         Novement 19, 2021             *
%         *****************************************
%
% This file extends the standard article class for
% the use with Rose-Hulman Undergraduate Mathematics Journal papers.

\NeedsTeXFormat{LaTeX2e}
\ProvidesFile{rhumj_new.cls}[2021/11/19]
\typeout{Rose-Hulman Undergraduate Mathematics Journal class -- version 4 (Nov. 19, 2021)}
\DeclareOption*{\PassOptionsToClass{\CurrentOption}{article}}
\ProcessOptions\relax

\LoadClass[12pt]{article}


%theorem,corollary,etc numbering, inner document referencing

\RequirePackage[colorlinks=true,
    pdfstartview=FitV,
    linkcolor=blue,
    citecolor=blue,
    urlcolor=blue,
    breaklinks=true,
    pdfencoding=auto,
    hyperfootnotes=false]{hyperref}

\RequirePackage{aliascnt}
\RequirePackage{amsthm,amssymb,amsmath}
\RequirePackage{cleveref}

\newtheorem{thm}{Theorem}[section]
    \crefformat{thm}{\bfseries theorem~#2#1#3}
    \Crefformat{thm}{\bfseries Theorem~#2#1#3}
\newtheorem{corollary}[thm]{Corollary}
    \crefformat{corollary}{\bfseries corollary~#2#1#3}
    \Crefformat{corollary}{\bfseries Corollary~#2#1#3}
\newtheorem{lemma}[thm]{Lemma}
    \crefformat{lemma}{\bfseries lemma~#2#1#3}
    \Crefformat{lemma}{\bfseries Lemma~#2#1#3}
\newtheorem{prop}[thm]{Proposition}
    \crefformat{prop}{\bfseries proposition~#2#1#3}
    \Crefformat{prop}{\bfseries Proposition~#2#1#3}

\theoremstyle{definition}
\newaliascnt{defin}{thm}
\newtheorem{defin}[thm]{Definition}
    \crefformat{defin}{\bfseries definition~#2#1#3}
    \Crefformat{defin}{\bfseries Definition~#2#1#3}
\newtheorem{remark}[thm]{Remark}
\newtheorem{exa}[thm]{Example}
    \crefformat{exa}{\bfseries example~#2#1#3}
    \Crefformat{exa}{\bfseries Example~#2#1#3}

\newtheorem*{xrem}{Remark}

%page setup

\RequirePackage[letterpaper]{geometry}
\geometry{
    asymmetric, %margins are not swapped
    hmarginratio=1:1,
    headheight=0.5in,
    footskip=0.5in,
    bottom=1.5in}

% report number


\IfFileExists{pubinfo.tex}{
  \input{pubinfo.tex}
  }{
  \gdef\@vol{Volume $\infty$}
  \gdef\@iss{Issue $\pi$}
  \gdef\@yr{2112}
}

% header/footer style

\renewcommand*{\title}[2][]{\gdef\shorttitle{#1}\gdef\@title{#2}}
\edef\title{\@nx\@dblarg
  \@xp\@nx\csname\string\title\endcsname}

\newcounter{numauth}
\setcounter{numauth}{0}

\renewcommand{\author}[2][]{%
  \ifx\@empty\authors
    \gdef\authors{\textit{#2}}%
    \g@addto@macro\contactinfos{\textbf{#2}}%
  \else
    \g@addto@macro\authors{\and\textit{#2}}%
    \g@addto@macro\contactinfos{\textbf{#2}}%
  \fi
  \@ifnotempty{#1}{%
    \ifx\@empty\shortauthors
      \gdef\shortauthors{#1}%
    \else
      \g@addto@macro\shortauthors{\and#1}%
    \fi
  }%
}
\edef\author{\@nx\@dblarg
  \@xp\@nx\csname\string\author\endcsname}
\let\shortauthors\@empty   \let\authors\@empty

\newcommand{\xandlist}[4]{\@andlista{{#1}{#2}{#3}}#4\and\and}
\def\@andlista#1#2\and#3\and{\@andlistc{#2}\@ifnotempty{#3}{%
  \@andlistb#1{#3}}}
\def\@andlistb#1#2#3#4#5\and{%
  \@ifempty{#5}{%
    \@andlistc{#2#4}%
  }{%
    \@andlistc{#1#4}\@andlistb{#1}{#3}{#3}{#5}%
  }}
\let\@andlistc\@iden
\newcommand{\nxandlist}[4]{%
  \def\@andlistc##1{\toks@\@xp{\the\toks@##1}}%
  \toks@{\toks@\@emptytoks \@andlista{{#1}{#2}{#3}}}%
  \the\@xp\toks@#4\and\and
  \edef#4{\the\toks@}%
  \let\@andlistc\@iden}
\def\@@and{and}
\newcommand{\andify}{%
  \nxandlist{\unskip, }{\unskip{} \@@and~}{\unskip, \@@and~}}
\def\and{\unskip{ }\@@and{ }\ignorespaces}

\RequirePackage{fancyhdr}
    \pagestyle{fancy}
    \fancyhead{}
    \fancyhead[RO,LE]{\footnotesize \thepage}
    \fancyhead[CE]{\shorttitle}
    \fancyhead[CO]{\author@andify\shortauthors \shortauthors}
    \lfoot{\footnotesize Rose-Hulman Undergrad. Math. J.}
    \cfoot{}
    \rfoot{\footnotesize \@vol, \@iss, \@yr}
    \renewcommand{\headrulewidth}{0pt}
    \renewcommand{\footrulewidth}{0pt}

%section/subsection/etc style

\RequirePackage{titlesec}
\titleformat{\section}{\centering\large\bfseries}
    {\thesection}{1em}{}
\titleformat*{\subsection}{\bfseries}
\titleformat{\subsubsection}[runin]
    {\normalfont\normalsize\bfseries}
    {\thesubsubsection}{1em}{}[.]

%graphics

\RequirePackage{graphicx}
\RequirePackage{tikz}

\RequirePackage{xcolor}
\definecolor{rose}{RGB}{128,0,0}
\definecolor{rosenavy}{RGB}{79,117,139}
\definecolor{rosegray}{RGB}{84,88,90}

%frontmatter

\gdef\@abstract{Place abstract here.}
\renewcommand{\abstract}[1]{\def\@abstract{#1}}

\gdef\@keywords{Place keywords here.}
\newcommand{\keywords}[1]{\def\@keywords{#1}}

\gdef\@subjclass{Subject classification.}
\newcommand{\subjclass}[1]{\def\@subjclass{#1}}

\AtBeginDocument{
\begin{tikzpicture}[overlay,remember picture]
    \node [rectangle,
        fill=rosenavy,
        anchor=north,
        minimum width=\paperwidth,
        minimum height=0.1\paperheight]
        (box) at (current page.north)
        {};
    \node [anchor=center]
        (title) at (box.center)
        {\color{white}
        {\Huge \bfseries $\mathbb{R}$ose-Hulman Undergraduate Mathematics Journal}};
    \node[anchor=north] 
        at (title.south)
        {\color{white}
        {\large \textsc{ \@vol, \@iss, \@yr }}};
\end{tikzpicture}
\def\author@andify{%
  \nxandlist {\unskip ,\penalty-1 \space\ignorespaces}%
    {\unskip {} \@@and~}%
    {\unskip ,\penalty-2 \space \@@and~}%
}
\thispagestyle{empty}
\begin{center}
    {\LARGE\bfseries \@title}
    \vskip 0.5cm
    \rule{2in}{1pt}
    \vskip 0.5cm
    \begingroup
        \renewcommand\thefootnote{\@fnsymbol\c@footnote}%
        \def\@makefnmark{\rlap{\@textsuperscript{\normalfont\@thefnmark}}}%
        By \author@andify\authors \authors
    \endgroup
    \setcounter{footnote}{0}%
\end{center}
\begingroup
        \renewcommand\thefootnote{\@fnsymbol\c@footnote}%
        \def\@makefnmark{\rlap{\@textsuperscript{\normalfont\@thefnmark}}}%
        \long\def\@makefntext#1{\parindent 1em\noindent
                \hb@xt@1.8em{%
                    \hss\@textsuperscript{\normalfont\@thefnmark}}#1}%
        \thispagestyle{plain}\@thanks
\endgroup
%\noindent {\bfseries Abstract.} \@abstract
\begingroup
    \renewcommand\thefootnote{}\footnote{%
    {\itshape Mathematics Subject Classification.} \@subjclass
    }
    \addtocounter{footnote}{-1}%
\endgroup
\begingroup
    \renewcommand\thefootnote{}\footnote{%
    {\itshape Keywords.} \@keywords
    }
    \addtocounter{footnote}{-1}%
\endgroup
\global\let\@thanks\@empty
\vskip0pt
}

%font

\RequirePackage[upright]{fourier}
\RequirePackage{microtype}

%back matter

\let\contactinfos\@empty
\newcommand{\affiliation}[1]{%
  \g@addto@macro\contactinfos{\affiliation{#1}}%
}
\newcommand{\email}[1]{%
  \g@addto@macro\contactinfos{\email{#1}}%
}
\def\enddoc@text{
    \ifx\@empty\contactinfos%
    \else\@setcontactinfos%
    \fi%
}
\AtEndDocument{
    \renewcommand\thefootnote{}%
    \def\@makefnmark{}%
    \setlength{\bigskipamount}{1cm}
    \enddoc@text
}
\def\@setcontactinfos{\par%
    \nobreak \begingroup %
    \interlinepenalty\@M
    \def\affiliation##1{\begingroup
      \@ifnotempty{##1}{\par\ignorespaces##1\unskip}\par\endgroup}%
    \def\email##1{\begingroup
      \@ifnotempty{##1}{\par\ignorespaces\ttfamily##1\unskip}\par\endgroup \vspace*{0.5cm}}%%
    \contactinfos%
    \endgroup%
}%
\renewcommand{\abstract}[1]{%
  \begingroup\noindent \bf Abstract. \endgroup%
  \begingroup\footnotesize\color{rosegray} #1 \endgroup%
}

\endinput