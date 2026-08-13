# Formula Rendering Test

This page is a rendering test for hand-translating the inference rules from
`latex/translating.tex` (Fig. 2, the $\mathcal{T}$ function) into the book
via the `mdbook-katex` preprocessor. Only a few representative rules are
shown.

The macros from `latex/macros.tex` are ported once into
`docs/katex-macros.txt` and are available on every page, so the rule bodies
below stay nearly identical to the paper's source.

## Axioms

$$
\begin{array}{cc}
\text{Int} & \text{String} \\[4pt]
\dfrac{}{\typcomp{\cint} \defeq \rint}
&
\dfrac{}{\typcomp{\cstring} \defeq \rstring}
\end{array}
$$

## Rule with premises

$$
\begin{array}{c}\text{Unique Pointer}\\[4pt]
\dfrac{
    \typcomp{\typ} = \rtyp
    \qquad
    \boxTy(\rtyp) = \rtyp'
}{\typcomp{\uniqueptr{\typ}} \defeq \roption{\rtyp'}}
\end{array}
$$

## Rule with premises on two lines

$$
\begin{array}{c}\text{Ptr - Non-Virtual Class}\\[4pt]
\dfrac
   {\begin{array}{c}
      \typcomp{\typ} = \rtyp
      \qquad
      \typ \text{ is not a virtual class}
      \\
      \converttoptr(\rtyp) = \rtyp'
   \end{array}}{\typcomp{\typ*} \defeq \ptr{\rtyp'}}
\end{array}
$$
