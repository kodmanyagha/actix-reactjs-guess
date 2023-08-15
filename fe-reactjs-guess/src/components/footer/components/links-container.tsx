import { Link } from "react-router-dom";

export type LinksContainerProps = {
  title: string,
  links: {
    title: string, url: string
  }[]
}

export default function LinksContainer(props: LinksContainerProps) {
  return (
    <>
      <div className="col-6 col-md">
        <h5>{props.title}</h5>
        <ul className="list-unstyled text-small">
          {props.links.map((item, index) => (
            <li className="mb-1" key={index}>
              <Link
                to={item.url}
                className="link-secondary text-decoration-none"
              >
                {item.title}
              </Link>
            </li>
          ))}
        </ul>
      </div>
    </>
  );
}
