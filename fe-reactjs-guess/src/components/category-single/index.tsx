import { Link } from "react-router-dom";
import ImgFallback from "../img-fallback";

export type BoxContent = {
  id: number,
  name: string,
  slug: string,
  image: string,
}

export type CategorySingleProps = {
  category: BoxContent,
}

export default function CategorySingle(props: CategorySingleProps) {
  return (
    <>
      <div className="col-md-3" key={props.category.id}>
        <div className="card mb-4 rounded-3 shadow-sm border-primary">
          <div
            className="card-header py-3 text-bg-primary border-primary"
            style={{ height: "100px" }}
          >
            <h4 className="my-0 fw-normal">{props.category.name}</h4>
          </div>
          <div className="card-body">
            <ImgFallback
              src={props.category.image}
              style={{ width: "100%", margin: "10px", aspectRatio: 1 }}
            />

            <Link
              to={`/category/${props.category.slug}`}
              className="w-100 btn btn-lg btn-primary"
            >
              <i className="fa-solid fa-circle-arrow-right" />
              &nbsp; Details
            </Link>
          </div>
        </div>
      </div>
    </>
  );
}
