export type ImgFallbackProps = {
  src?: string,
  style?: object,
  className?: string
}

export default function ImgFallback(props: ImgFallbackProps) {
  return (
    <img
      src={props.src}
      style={props.style}
      className={props.className}
      onError={(e: React.SyntheticEvent<HTMLImageElement, Event>) => {
        e.currentTarget.src = "/assets/images/no-image.png";
      }}
    />
  );
}
